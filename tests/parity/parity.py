#!/usr/bin/env python3
"""Differential parity harness: Go reference server vs Rust server.

For every tool (optionally filtered), this script runs a matrix of argument
variants against a mock App Store Connect API in several response modes,
sends the identical tools/call to both servers over stdio, and compares:

  * every HTTP request each server made to the mock (method, path, query,
    relevant headers, JSON body / upload bytes), in order, and
  * the complete tools/call result (content, structuredContent, isError).

Mock responses are synthesized from Apple's OpenAPI document
(doc/references/apple-asc/openapi.oas.json), so each endpoint returns the
shape the real API returns.

Usage:
  tests/parity/parity.py --go BIN --rust BIN [--tools REGEX] [--modes rich,sparse]
                         [--jobs N] [--report out.json] [--verbose]

Building the Go reference after the Go sources were removed from the tree:
  tests/parity/build-go-reference.sh <git-rev-with-go-code> <out-binary>

Custom argument sets for a tool can be added in tests/parity/cases/<tool>.json
as a JSON array of argument objects (or objects with "args" and optional
"modes").
"""

from __future__ import annotations

import argparse
import base64
import gzip
import hashlib
import http.server
import json
import os
import queue
import re
import subprocess
import sys
import tempfile
import threading
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OPENAPI = ROOT / "doc/references/apple-asc/openapi.oas.json"
CASES_DIR = Path(__file__).resolve().parent / "cases"

MODES = ["rich", "alt", "sparse", "empty", "notfound", "rawerror"]

# --------------------------------------------------------------------------
# OpenAPI-driven response synthesis


class Spec:
    def __init__(self, path: Path):
        self.doc = json.loads(path.read_text())
        self.schemas = self.doc["components"]["schemas"]
        self.routes = []
        for tmpl, ops in self.doc["paths"].items():
            pattern = "^" + re.sub(r"\{[^}]+\}", "[^/]+", re.escape(tmpl).replace(r"\{", "{").replace(r"\}", "}")) + "$"
            literal = len(re.sub(r"\{[^}]+\}", "", tmpl))
            self.routes.append((re.compile(pattern), literal, tmpl, ops))
        # Prefer the most literal template (e.g. /v1/apps/{id}/builds over /v1/{x}/{y}/builds).
        self.routes.sort(key=lambda r: -r[1])

    def resolve(self, s):
        seen = 0
        while isinstance(s, dict) and "$ref" in s and seen < 20:
            s = self.schemas[s["$ref"].split("/")[-1]]
            seen += 1
        return s

    def operation(self, method: str, path: str):
        for rx, _lit, tmpl, ops in self.routes:
            if rx.match(path):
                op = ops.get(method.lower())
                if op is not None:
                    return tmpl, op
        return None, None


DATE_TIMES = {"rich": "2024-03-05T10:20:30.123-07:00", "alt": "2023-12-31T23:59:59Z"}


def gen(spec: Spec, s, mode: str, key: str, depth: int, ctx: dict):
    s = spec.resolve(s)
    if not isinstance(s, dict):
        return None
    for comb in ("oneOf", "anyOf"):
        if comb in s:
            opts = s[comb]
            return gen(spec, opts[ctx.get("pick", 0) % len(opts)], mode, key, depth, ctx)
    if "allOf" in s:
        out = {}
        for part in s["allOf"]:
            v = gen(spec, part, mode, key, depth, ctx)
            if isinstance(v, dict):
                out.update(v)
        return out
    if "enum" in s:
        return s["enum"][0] if mode != "alt" else s["enum"][-1]
    t = s.get("type")
    if t == "object" or "properties" in s:
        out = {}
        if depth > 7:
            return out
        for i, (k, v) in enumerate(s.get("properties", {}).items()):
            if mode == "alt" and i % 2 == 1 and k not in ("type", "id", "data"):
                continue
            if mode == "sparse" and depth >= 1 and k not in ("type", "id", "data", "links", "self"):
                continue
            out[k] = gen(spec, v, mode, k, depth + 1, ctx)
        return out
    if t == "array":
        n = {"rich": 2, "alt": 1}.get(mode, 1)
        items = []
        for i in range(n):
            sub = dict(ctx, pick=ctx.get("pick", 0) + i)
            items.append(gen(spec, s.get("items", {}), mode, key, depth + 1, sub))
        return items
    if t == "string":
        fmt = s.get("format", "")
        if key == "id":
            ctx["ids"] = ctx.get("ids", 0) + 1
            return f"res-{ctx['ids']}"
        if fmt == "date-time":
            return DATE_TIMES.get(mode, DATE_TIMES["rich"])
        if fmt == "date":
            return "2024-03-05"
        if fmt in ("uri", "uri-reference", "url"):
            return f"{ctx['base']}/asset/{key}"
        if fmt == "email":
            return "someone@example.com"
        if mode == "alt":
            return ""
        return f"{key}-S"
    if t == "integer":
        return 3 if mode != "alt" else 0
    if t == "number":
        return 2.5 if mode != "alt" else 0
    if t == "boolean":
        return mode != "alt"
    return None


def fix_uploads(v, base: str):
    if isinstance(v, dict):
        for k, x in list(v.items()):
            if k == "uploadOperations" and isinstance(x, list):
                v[k] = [
                    {
                        "method": "PUT",
                        "url": f"{base}/__upload/{i}",
                        "length": 1,
                        "offset": 0,
                        "requestHeaders": [{"name": "Content-Type", "value": "application/octet-stream"}],
                    }
                    for i in range(min(len(x), 1) or 1)
                ]
            else:
                fix_uploads(x, base)
    elif isinstance(v, list):
        for x in v:
            fix_uploads(x, base)


def synthesize(spec: Spec, method: str, path: str, mode: str, base: str):
    """Returns (status, content_type, body_bytes)."""
    if mode == "notfound":
        body = {"errors": [
            {"id": "e1", "status": "404", "code": "NOT_FOUND", "title": "The specified resource does not exist",
             "detail": f"There is no resource with path '{path}'."},
            {"status": "404", "code": "NOT_FOUND.SECOND", "title": "Second error", "detail": "More detail <&>"},
        ]}
        return 404, "application/json", json.dumps(body).encode()
    if mode == "rawerror":
        return 400, "text/plain", b"upstream rejected the request"
    tmpl, op = spec.operation(method, path)
    if method == "DELETE" and op is None:
        return 204, "", b""
    if op is None:
        last = path.rstrip("/").split("/")[-1]
        single = method != "GET" or last.endswith("-val") or last.startswith("res-")
        item = {"type": "resources", "id": "res-1", "attributes": {"name": "name-S"}}
        data = item if single else ([] if mode == "empty" else [item, dict(item, id="res-2")])
        return (201 if method == "POST" else 200), "application/json", json.dumps({"data": data, "links": {"self": base + path}}).encode()
    responses = op.get("responses", {})
    codes = sorted(c for c in responses if c.startswith("2"))
    code = codes[0] if codes else "200"
    content = responses.get(code, {}).get("content")
    if not content:
        return int(code), "", b""
    ctype, media = next(iter(content.items()))
    if "gzip" in ctype:
        tsv = "Provider\tSKU\tUnits\nAPPLE\tsku-S\t3\n" if mode != "empty" else ""
        return int(code), ctype, gzip.compress(tsv.encode(), mtime=0)
    if not ctype.endswith("json"):
        return int(code), ctype, f"{path}-body".encode()
    ctx = {"base": base}
    body = gen(spec, media.get("schema", {}), mode if mode != "empty" else "rich", "", 0, ctx)
    if isinstance(body, dict):
        if mode == "empty" and isinstance(body.get("data"), list):
            body["data"] = []
        if mode == "sparse" and isinstance(body.get("data"), list):
            body["data"] = body["data"][:1]
        links = body.get("links")
        if isinstance(links, dict):
            links["self"] = base + path
            if mode == "rich" and isinstance(body.get("data"), list):
                links["next"] = f"{base}{path}?cursor=Q1VSU09SLTI&limit=2&fields%5Bx%5D=a%2Cb"
            else:
                links.pop("next", None)
    fix_uploads(body, base)
    return int(code), ctype, json.dumps(body).encode()


# --------------------------------------------------------------------------
# Mock server


class Mock:
    def __init__(self, spec: Spec):
        self.spec = spec
        self.mode = "rich"
        self.log = []
        self.lock = threading.Lock()
        mock = self

        class Handler(http.server.BaseHTTPRequestHandler):
            protocol_version = "HTTP/1.1"

            def log_message(self, *a):
                pass

            def _handle(self):
                length = int(self.headers.get("Content-Length") or 0)
                raw = self.rfile.read(length) if length else b""
                path, _, query = self.path.partition("?")
                rec = {
                    "method": self.command,
                    "path": path,
                    "query": query,
                    "headers": {
                        k: self.headers.get(k)
                        for k in ("Content-Type", "Accept", "User-Agent")
                        if self.headers.get(k) is not None
                    },
                }
                auth = self.headers.get("Authorization")
                if auth is not None:
                    rec["auth"] = "Bearer <jwt>" if re.fullmatch(r"Bearer [\w-]+\.[\w-]+\.[\w-]+", auth) else auth
                if raw:
                    try:
                        rec["body"] = json.loads(raw)
                    except ValueError:
                        rec["body"] = {"bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}
                if path.startswith("/__upload/"):
                    # Intended differences on pre-signed upload PUTs: Go sends
                    # its default "Go-http-client/1.1" User-Agent and no Accept;
                    # the Rust server identifies itself and reqwest always adds
                    # "Accept: */*", which is equivalent to omitting Accept.
                    if rec["headers"].get("User-Agent") in ("Go-http-client/1.1", "asc-mcp/1.0.0 (+https://github.com/antisynthesis/asc-mcp)"):
                        rec["headers"]["User-Agent"] = "<upload user agent>"
                    if rec["headers"].get("Accept") == "*/*":
                        rec["headers"].pop("Accept")
                with mock.lock:
                    mock.log.append(rec)
                    mode = mock.mode
                if path.startswith("/__upload/"):
                    status, ctype, body = 200, "", b""
                else:
                    status, ctype, body = synthesize(mock.spec, self.command, path, mode, mock.base)
                self.send_response(status)
                if ctype:
                    self.send_header("Content-Type", ctype)
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                if body:
                    self.wfile.write(body)

            do_GET = do_POST = do_PATCH = do_DELETE = do_PUT = _handle

        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.base = f"http://127.0.0.1:{self.server.server_address[1]}"
        threading.Thread(target=self.server.serve_forever, daemon=True).start()

    def reset(self, mode: str):
        with self.lock:
            self.mode = mode
            self.log = []

    def take(self):
        with self.lock:
            out, self.log = self.log, []
            return out


# --------------------------------------------------------------------------
# MCP stdio client


class Server:
    def __init__(self, cmd: list[str], env: dict):
        self.proc = subprocess.Popen(
            cmd + ["serve"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env=env
        )
        self.q: queue.Queue = queue.Queue()
        threading.Thread(target=self._read, daemon=True).start()
        self.next_id = 0
        r = self.request("initialize", {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "parity", "version": "1"}})
        assert "result" in r, r
        self.notify("notifications/initialized")

    def _read(self):
        for line in self.proc.stdout:
            self.q.put(json.loads(line))
        self.q.put(None)

    def send(self, msg: dict):
        self.proc.stdin.write((json.dumps(msg) + "\n").encode())
        self.proc.stdin.flush()

    def notify(self, method: str):
        self.send({"jsonrpc": "2.0", "method": method})

    def request(self, method: str, params, timeout=60):
        self.next_id += 1
        msg = {"jsonrpc": "2.0", "id": self.next_id, "method": method}
        if params is not None:
            msg["params"] = params
        self.send(msg)
        deadline = time.time() + timeout
        while True:
            m = self.q.get(timeout=max(0.1, deadline - time.time()))
            if m is None:
                raise RuntimeError("server exited")
            if m.get("id") == self.next_id:
                return m

    def close(self):
        try:
            self.proc.stdin.close()
            self.proc.wait(timeout=5)
        except Exception:
            self.proc.kill()


# --------------------------------------------------------------------------
# Argument variants


def sample(name: str, prop: dict, files: dict):
    t = prop.get("type")
    if "enum" in prop:
        return prop["enum"][0]
    if name == "file_path":
        return files["path"]
    if name == "file_data_base64":
        return files["b64"]
    if t == "string":
        if name == "cursor":
            return None
        if "date" in name or name.endswith("_at") or "time" in name:
            return "2024-03-05T10:20:30Z"
        if "url" in name:
            return "https://example.com/" + name
        if "email" in name:
            return "dev@example.com"
        return f"{name}-val"
    if t == "integer":
        lo, hi = prop.get("minimum"), prop.get("maximum")
        v = 7
        if lo is not None and v < lo:
            v = int(lo)
        if hi is not None and v > hi:
            v = int(hi)
        return v
    if t == "number":
        return 4.99
    if t == "boolean":
        return True
    if t == "array":
        it = prop.get("items", {})
        if it.get("type") == "object" or "properties" in it:
            return [{k: sample(k, p, files) for k, p in it.get("properties", {}).items()} or {"k1": ["v1"]}]
        return [sample(f"{name}0", it, files) or "a1", sample(f"{name}1", it, files) or "b2"] if it.get("type") in ("string", None) else [sample(name, it, files)] * 2
    if t == "object":
        if prop.get("properties"):
            return {k: sample(k, p, files) for k, p in prop["properties"].items()}
        return {"platform": ["IOS", "MAC_OS"], "k2": ["v2"]}
    return f"{name}-val"


def zero(prop: dict):
    return {"string": "", "integer": 0, "number": 0, "boolean": False, "array": [], "object": {}}.get(prop.get("type"), None)


def wrong(prop: dict):
    return {"string": 123, "integer": "x", "number": "x", "boolean": "x", "array": "x", "object": "x"}.get(prop.get("type"), 5)


def variants(tool: dict, files: dict, base: str):
    schema = tool["inputSchema"]
    props = schema.get("properties", {})
    req = schema.get("required", [])
    full = {k: v for k, v in ((k, sample(k, p, files)) for k, p in props.items()) if v is not None}
    out = []
    if "file_path" in full and "file_data_base64" in full:
        a = dict(full)
        a.pop("file_data_base64")
        b = dict(full)
        b.pop("file_path")
        out += [("full-path", a, None), ("full-b64", b, None), ("full-both", full, ["rich"])]
    else:
        out.append(("full", full, None))
    out.append(("required", {k: full[k] for k in req if k in full}, None))
    out.append(("empty", {}, ["rich"]))
    out.append(("absent", None, ["rich"]))
    out.append(("zero", {k: zero(p) for k, p in props.items()}, ["rich", "sparse"]))
    out.append(("wrong", {k: wrong(p) for k, p in props.items()}, ["rich"]))
    if "cursor" in props:
        c = dict(full, cursor=f"{base}/v1/cursorPage?cursor=Q1VSU09S&limit=5")
        out.append(("cursor", c, ["rich", "notfound"]))
        out.append(("cursor-foreign", dict(full, cursor="https://evil.example.com/v1/apps?limit=1"), ["rich"]))
    extra = CASES_DIR / f"{tool['name']}.json"
    if extra.exists():
        for i, case in enumerate(json.loads(extra.read_text())):
            if isinstance(case, dict) and "args" in case:
                out.append((case.get("name", f"custom{i}"), case["args"], case.get("modes")))
            else:
                out.append((f"custom{i}", case, None))
    return out


def expand(v, files):
    """Replaces $FILE / $B64 placeholders in custom cases."""
    if isinstance(v, str):
        return v.replace("$FILE", files["path"]).replace("$B64", files["b64"])
    if isinstance(v, list):
        return [expand(x, files) for x in v]
    if isinstance(v, dict):
        return {k: expand(x, files) for k, x in v.items()}
    return v


# --------------------------------------------------------------------------
# Runner


def run_shard(tools, args, spec, files, shard_id, results):
    mock = Mock(spec)
    env = dict(os.environ)
    env.update({
        "ASC_API_BASE_URL": mock.base,
        "ASC_ISSUER_ID": "11111111-2222-3333-4444-555555555555",
        "ASC_KEY_ID": "PARITYKEY1",
        "ASC_PRIVATE_KEY_PATH": files["key"],
        "TZ": "UTC",
    })
    go = Server(args.go, env)
    rs = Server(args.rust, env)
    modes = args.modes.split(",")
    try:
        for tool in tools:
            for vname, a, vmodes in variants(tool, files, mock.base):
                a = expand(a, files)
                for mode in modes:
                    if vmodes is not None and mode not in vmodes:
                        continue
                    params = {"name": tool["name"]}
                    if a is not None:
                        params["arguments"] = a
                    mock.reset(mode)
                    gr = go.request("tools/call", params)
                    glog = mock.take()
                    rr = rs.request("tools/call", params)
                    rlog = mock.take()
                    case = {"tool": tool["name"], "variant": vname, "mode": mode, "args": a}
                    diffs = []
                    if gr.get("result") != rr.get("result") or gr.get("error") != rr.get("error"):
                        diffs.append({"what": "response", "go": {k: gr.get(k) for k in ("result", "error")}, "rust": {k: rr.get(k) for k in ("result", "error")}})
                    if glog != rlog:
                        diffs.append({"what": "requests", "go": glog, "rust": rlog})
                    case["ok"] = not diffs
                    if diffs:
                        case["diffs"] = diffs
                    case["requests"] = len(glog)
                    results.append(case)
    finally:
        go.close()
        rs.close()
        mock.server.shutdown()


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--go", required=True, help="Go reference binary")
    ap.add_argument("--rust", required=True, help="Rust binary")
    ap.add_argument("--tools", default=".*", help="regex of tool names to test")
    ap.add_argument("--modes", default=",".join(MODES))
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 4)
    ap.add_argument("--report", help="write a JSON report here")
    ap.add_argument("--verbose", "-v", action="store_true", help="print full diffs")
    ap.add_argument("--max-diffs", type=int, default=5, help="diffs to print per tool")
    args = ap.parse_args()
    args.go = [args.go]
    args.rust = [args.rust]

    spec = Spec(OPENAPI)
    tmp = Path(tempfile.mkdtemp(prefix="parity-"))
    key = tmp / "key.p8"
    subprocess.run("openssl ecparam -name prime256v1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt -out " + str(key),
                   shell=True, check=True, stderr=subprocess.DEVNULL)
    payload = b"\x89PNG\r\n\x1a\nparity-test-payload"
    fpath = tmp / "asset.png"
    fpath.write_bytes(payload)
    files = {"key": str(key), "path": str(fpath), "b64": base64.b64encode(payload).decode()}

    # Tool definitions from the Rust server (and compare with Go's).
    env = dict(os.environ, ASC_ISSUER_ID="i", ASC_KEY_ID="k", ASC_PRIVATE_KEY_PATH=str(key), ASC_API_BASE_URL="http://127.0.0.1:9")
    g, r = Server(args.go, env), Server(args.rust, env)
    gt, rt = g.request("tools/list", {})["result"]["tools"], r.request("tools/list", {})["result"]["tools"]
    g.close()
    r.close()
    defs_ok = gt == rt
    print(f"tools/list: go={len(gt)} rust={len(rt)} identical={defs_ok}")
    if not defs_ok:
        for a, b in zip(gt, rt):
            if a != b:
                print(f"  first difference at tool {a.get('name')} vs {b.get('name')}")
                break

    rx = re.compile(args.tools)
    tools = [t for t in gt if rx.fullmatch(t["name"])]
    shards = [tools[i::args.jobs] for i in range(args.jobs)]
    results: list = []
    with ThreadPoolExecutor(max_workers=args.jobs) as ex:
        futs = [ex.submit(run_shard, s, args, spec, files, i, results) for i, s in enumerate(shards) if s]
        for f in futs:
            f.result()

    by_tool: dict = {}
    for c in results:
        by_tool.setdefault(c["tool"], []).append(c)
    failing = {t: [c for c in cs if not c["ok"]] for t, cs in by_tool.items()}
    failing = {t: cs for t, cs in failing.items() if cs}
    total = len(results)
    passed = sum(1 for c in results if c["ok"])
    for t in sorted(failing):
        cs = failing[t]
        print(f"FAIL {t}: {len(cs)}/{len(by_tool[t])} cases differ")
        for c in cs[: args.max_diffs]:
            print(f"  - variant={c['variant']} mode={c['mode']} args={json.dumps(c['args'])[:300]}")
            for d in c["diffs"]:
                if args.verbose:
                    print(f"    {d['what']}:\n      go:   {json.dumps(d['go'])}\n      rust: {json.dumps(d['rust'])}")
                else:
                    print(f"    {d['what']}:\n      go:   {json.dumps(d['go'])[:700]}\n      rust: {json.dumps(d['rust'])[:700]}")
    exercised = sum(1 for c in results if c["requests"])
    print(f"\n{passed}/{total} cases identical across {len(by_tool)} tools "
          f"({len(by_tool) - len(failing)} tools fully identical); {exercised} cases made API requests")
    if args.report:
        Path(args.report).write_text(json.dumps({"defs_identical": defs_ok, "results": results}, indent=1))
    sys.exit(0 if (not failing and defs_ok) else 1)


if __name__ == "__main__":
    main()
