#!/usr/bin/env python3
"""DEPRECATED: scheduled for removal with the handshake protocol revisions.

Protocol-level differential check: Go reference vs Rust server.

Runs scripted JSON-RPC conversations over stdio and scripted HTTP exchanges
against `serve-http`, on both binaries, and compares the responses
(session ids and other random values are normalized). Scenarios where the
Rust server intentionally follows MCP 2026-07-28 more strictly than the Go
server are marked `expect_diff` with the reason; for those both outputs are
printed for review instead of being compared.

Usage: tests/parity/protocol.py --go BIN --rust BIN
"""

from __future__ import annotations

import argparse
import json
import os
import re
import socket
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request

M = {"io.modelcontextprotocol/protocolVersion": "2026-07-28", "io.modelcontextprotocol/clientCapabilities": {}}
LEGACY_INIT = {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": {"name": "t", "version": "1"}}}


def req(i, method, params=None, **extra):
    m = {"jsonrpc": "2.0", "id": i, "method": method}
    if params is not None:
        m["params"] = params
    m.update(extra)
    return m


def note(method, params=None):
    m = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        m["params"] = params
    return m


# (name, lines (dicts or raw strings), expect_diff_reason)
STDIO = [
    ("initialize-versions", [req(1, "initialize", {"protocolVersion": v}) for v in ["2024-11-05", "2025-03-26", "2025-06-18", "2025-11-25", "2026-07-28", "1999-01-01", ""]], None),
    ("initialize-bad-params", [req(1, "initialize", {"protocolVersion": 5}), req(2, "initialize", "x"), req(3, "initialize", {"capabilities": {"roots": {"listChanged": "yes"}}})], None),
    ("not-initialized", [req(1, "tools/list"), req(2, "tools/call", {"name": "get_app"})], None),
    ("legacy-basics", [LEGACY_INIT, note("notifications/initialized"), req(2, "ping"), req(3, "nope"), note("nope"), req("s", "ping"), req(None, "ping"),
                       req(4, "tools/call", {"name": "missing_tool"}), req(5, "tools/call", {"name": "get_app", "arguments": {}}),
                       req(6, "tools/call", {"name": "get_app"}), req(7, "tools/call", {"name": "get_app", "arguments": {"app_id": 5}}),
                       req(8, "tools/call", {"name": 5}), req(9, "tools/call"), req(10, "tools/call", {"name": "get_app", "arguments": None}),
                       req(11, "tools/call", {"name": "get_app", "arguments": [1]})], None),
    ("legacy-tools-list", [LEGACY_INIT, req(2, "tools/list")], None),
    ("framing", ["not json", "[1,2]", '{"jsonrpc":"1.0","id":1,"method":"ping"}', '{"jsonrpc":"2.0","id":1,"method":5}', "", "  ", '{"jsonrpc":"2.0","id":2,"method":"ping"}'], None),
    ("modern-basics", [req(1, "tools/list", {"_meta": M}), req(2, "ping", {"_meta": M}), req(3, "logging/setLevel", {"_meta": M, "level": "info"}),
                       req(4, "tools/call", {"_meta": M, "name": "missing_tool"}), req(5, "tools/call", {"_meta": M, "name": "get_app", "arguments": {}}),
                       req(6, "tools/call", {"_meta": {"io.modelcontextprotocol/protocolVersion": "2026-07-28"}, "name": "get_app"}),
                       req(7, "tools/list", {"_meta": {"io.modelcontextprotocol/protocolVersion": "2099-01-01", "io.modelcontextprotocol/clientCapabilities": {}}}),
                       note("notifications/cancelled", {"_meta": M, "requestId": 99})], None),
    ("modern-discover", [req(1, "server/discover"), req(2, "server/discover", {"_meta": M})], None),
    ("modern-discover-bad-version", [req(1, "server/discover", {"_meta": {"io.modelcontextprotocol/protocolVersion": "2099-01-01", "io.modelcontextprotocol/clientCapabilities": {}}})],
     "spec: discover with an unsupported version MUST return UnsupportedProtocolVersionError"),
    ("modern-legacy-version-in-meta", [req(1, "tools/list", {"_meta": {"io.modelcontextprotocol/protocolVersion": "2025-11-25", "io.modelcontextprotocol/clientCapabilities": {}}})],
     "clearer -32022 message for a legacy version sent via _meta"),
    ("modern-strictness", [req(1, "tools/list", {"_meta": M, "cursor": "x"}), req(2, "tools/call", {"_meta": M, "name": "get_app", "arguments": [1]}),
                           req(3, "tools/list", {"_meta": dict(M, **{"io.modelcontextprotocol/clientCapabilities": None})})],
     "spec: invalid cursor / non-object arguments / malformed capabilities are -32602"),
    ("modern-listen", [req(1, "subscriptions/listen", {"_meta": M, "notifications": {"toolsListChanged": True}})], "spec: subscriptions/listen is a required message pattern"),
]


def run_stdio(binary, lines, env):
    p = subprocess.run([binary, "serve"], input="".join((l if isinstance(l, str) else json.dumps(l)) + "\n" for l in lines).encode(),
                       capture_output=True, env=env, timeout=60)
    out = [json.loads(l) for l in p.stdout.decode().splitlines() if l.strip()]
    # Concurrency may reorder responses; compare as a sorted multiset.
    return sorted(out, key=lambda m: json.dumps(m, sort_keys=True))


def free_port():
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    port = s.getsockname()[1]
    s.close()
    return port


def http(base, method, path="/mcp", body=None, headers=None):
    data = None if body is None else (body if isinstance(body, bytes) else json.dumps(body).encode())
    r = urllib.request.Request(base + path, data=data, method=method, headers=headers or {})
    try:
        with urllib.request.urlopen(r, timeout=30) as resp:
            status, h, b = resp.status, resp.headers, resp.read()
    except urllib.error.HTTPError as e:
        status, h, b = e.code, e.headers, e.read()
    # Header names are case-insensitive (hyper lowercases them, Go
    # canonicalizes them), so compare them lowercased.
    keep = {k.lower(): v for k, v in h.items() if k.lower() in ("content-type", "mcp-protocol-version", "allow", "www-authenticate", "mcp-session-id")}
    if "mcp-session-id" in keep:
        keep["mcp-session-id"] = "<session>" if re.fullmatch(r"[0-9a-f]{32}", keep["mcp-session-id"]) else keep["mcp-session-id"]
    text = b.decode(errors="replace")
    try:
        parsed = json.loads(text)
    except ValueError:
        parsed = text
    return {"status": status, "headers": keep, "body": parsed}, h.get("Mcp-Session-Id")


def H(method, name=None, pv="2026-07-28", **extra):
    h = {"Content-Type": "application/json", "Accept": "application/json, text/event-stream", "Mcp-Method": method}
    if pv is not None:
        h["MCP-Protocol-Version"] = pv
    if name is not None:
        h["Mcp-Name"] = name
    h.update(extra)
    return h


def http_scenarios(base):
    """Yields (name, result, expect_diff_reason)."""
    r, sid = http(base, "POST", body=LEGACY_INIT, headers={"Content-Type": "application/json"})
    yield "legacy-initialize", r, None
    yield "legacy-no-session", http(base, "POST", body=req(2, "tools/list"))[0], None
    yield "legacy-unknown-session", http(base, "POST", body=req(2, "tools/list"), headers={"Mcp-Session-Id": "nope"})[0], None
    yield "legacy-ping", http(base, "POST", body=req(3, "ping"), headers={"Mcp-Session-Id": sid})[0], None
    yield "legacy-notification", http(base, "POST", body=note("notifications/initialized"), headers={"Mcp-Session-Id": sid})[0], None
    yield "legacy-bad-pv", http(base, "POST", body=req(4, "ping"), headers={"Mcp-Session-Id": sid, "MCP-Protocol-Version": "1999-01-01"})[0], None
    yield "legacy-tool-error", http(base, "POST", body=req(5, "tools/call", {"name": "get_app", "arguments": {}}), headers={"Mcp-Session-Id": sid})[0], None
    yield "delete-no-header", http(base, "DELETE")[0], None
    yield "delete-session", http(base, "DELETE", headers={"Mcp-Session-Id": sid})[0], None
    yield "delete-again", http(base, "DELETE", headers={"Mcp-Session-Id": sid})[0], None
    yield "get", http(base, "GET")[0], None
    yield "options", http(base, "OPTIONS")[0], None
    yield "put", http(base, "PUT", body=b"x")[0], None
    yield "parse-error", http(base, "POST", body=b"{nope")[0], None
    yield "healthz", http(base, "GET", "/healthz")[0], None
    yield "not-found", http(base, "GET", "/other")[0], None
    yield "modern-tools-list", http(base, "POST", body=req(1, "tools/list", {"_meta": M}), headers=H("tools/list"))[0], None
    yield "modern-call", http(base, "POST", body=req(1, "tools/call", {"_meta": M, "name": "get_app", "arguments": {}}), headers=H("tools/call", "get_app"))[0], None
    yield "modern-call-b64-name", http(base, "POST", body=req(1, "tools/call", {"_meta": M, "name": "get_app", "arguments": {}}), headers=H("tools/call", "=?base64?Z2V0X2FwcA==?="))[0], None
    yield "modern-name-mismatch", http(base, "POST", body=req(1, "tools/call", {"_meta": M, "name": "get_app", "arguments": {}}), headers=H("tools/call", "list_apps"))[0], "richer Header mismatch message"
    yield "modern-method-mismatch", http(base, "POST", body=req(1, "tools/list", {"_meta": M}), headers=H("tools/call"))[0], "richer Header mismatch message"
    yield "modern-unknown-method", http(base, "POST", body=req(1, "nope", {"_meta": M}), headers=H("nope"))[0], None
    yield "modern-unknown-tool", http(base, "POST", body=req(1, "tools/call", {"_meta": M, "name": "zzz"}), headers=H("tools/call", "zzz"))[0], None
    yield "modern-notification", http(base, "POST", body=note("notifications/cancelled", {"_meta": M, "requestId": 1}), headers=H("notifications/cancelled"))[0], None
    yield "modern-session-ignored", http(base, "POST", body=req(1, "tools/list", {"_meta": M}), headers=H("tools/list", **{"Mcp-Session-Id": "abc"}))[0], None
    yield "modern-discover", http(base, "POST", body=req(1, "server/discover", {"_meta": M}), headers=H("server/discover"))[0], None
    yield "modern-discover-pv-mismatch", http(base, "POST", body=req(1, "server/discover", {"_meta": M}), headers=H("server/discover", pv="2025-11-25"))[0], "spec: MCP-Protocol-Version MUST match _meta on every POST"
    yield "modern-header-only", http(base, "POST", body=req(1, "tools/list"), headers=H("tools/list"))[0], "spec: header-selected modern request gets a JSON-RPC -32602, not a session error"
    yield "origin-foreign", http(base, "POST", body=req(1, "tools/list", {"_meta": M}), headers=H("tools/list", Origin="https://evil.example"))[0], "spec: Origin MUST be validated"
    yield "origin-loopback", http(base, "POST", body=req(1, "tools/list", {"_meta": M}), headers=H("tools/list", Origin="http://localhost:3000"))[0], None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--go", required=True)
    ap.add_argument("--rust", required=True)
    a = ap.parse_args()
    tmp = tempfile.mkdtemp()
    key = os.path.join(tmp, "k.p8")
    subprocess.run(f"openssl ecparam -name prime256v1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt -out {key}", shell=True, check=True, stderr=subprocess.DEVNULL)
    env = dict(os.environ, ASC_ISSUER_ID="11111111-2222-3333-4444-555555555555", ASC_KEY_ID="K", ASC_PRIVATE_KEY_PATH=key, ASC_API_BASE_URL="http://127.0.0.1:9", TZ="UTC")
    fails = 0
    for name, lines, reason in STDIO:
        g, r = run_stdio(a.go, lines, env), run_stdio(a.rust, lines, env)
        same = g == r
        if reason:
            print(f"[{'same' if same else 'intended diff'}] stdio/{name}: {reason}")
            if not same:
                print(f"    go:   {json.dumps(g)[:600]}\n    rust: {json.dumps(r)[:600]}")
        elif not same:
            fails += 1
            print(f"[FAIL] stdio/{name}\n    go:   {json.dumps(g)}\n    rust: {json.dumps(r)}")
        else:
            print(f"[ok] stdio/{name}")
    results = {}
    for label, binary in (("go", a.go), ("rust", a.rust)):
        port = free_port()
        p = subprocess.Popen([binary, "serve-http", "--addr", f"127.0.0.1:{port}", "--log-format", "text"], env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        base = f"http://127.0.0.1:{port}"
        for _ in range(100):
            try:
                urllib.request.urlopen(base + "/healthz", timeout=1)
                break
            except Exception:
                time.sleep(0.1)
        results[label] = list(http_scenarios(base))
        p.terminate()
        p.wait()
    for (name, g, reason), (_, r, _) in zip(results["go"], results["rust"]):
        same = g == r
        if reason:
            print(f"[{'same' if same else 'intended diff'}] http/{name}: {reason}")
            if not same:
                print(f"    go:   {json.dumps(g)[:500]}\n    rust: {json.dumps(r)[:500]}")
        elif not same:
            fails += 1
            print(f"[FAIL] http/{name}\n    go:   {json.dumps(g)}\n    rust: {json.dumps(r)}")
        else:
            print(f"[ok] http/{name}")
    print(f"\n{fails} unintended differences")
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
