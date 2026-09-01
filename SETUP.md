# Setup

How to build `asc-mcp`, get App Store Connect credentials, and register the
server with Claude Desktop, Claude Code, the Codex app, and the Codex CLI.

- [1. Install the binary](#1-install-the-binary)
- [2. Get App Store Connect credentials](#2-get-app-store-connect-credentials)
- [3. Verify the configuration](#3-verify-the-configuration)
- [4. Claude Desktop](#4-claude-desktop)
- [5. Claude Code (CLI)](#5-claude-code-cli)
- [6. Codex (app, CLI, and IDE extension)](#6-codex-app-cli-and-ide-extension)
- [7. Running over HTTP instead of stdio](#7-running-over-http-instead-of-stdio)
- [Troubleshooting](#troubleshooting)

Every client below launches the same binary the same way: `asc-mcp serve`,
speaking JSON-RPC over stdin/stdout, with three environment variables for
credentials. If you understand that one line, the rest is just each client's
config file format.

## 1. Install the binary

```bash
git clone https://github.com/antisynthesis/asc-mcp.git
cd asc-mcp
make install
```

`make install` builds the binary and copies it to `$GOPATH/bin` (defaults to
`~/go/bin`). Override the destination with `INSTALL_DIR`:

```bash
INSTALL_DIR=/usr/local/bin make install
```

`make build` alone leaves the binary at `./bin/asc-mcp`.

Confirm the install and note the absolute path — you need it for every client
config below:

```bash
which asc-mcp        # e.g. /Users/you/go/bin/asc-mcp
asc-mcp version
```

**Use the absolute path in client configs.** Desktop applications do not
inherit your shell's `PATH`, so a bare `asc-mcp` often fails to launch even
though it works in your terminal.

## 2. Get App Store Connect credentials

1. Open [App Store Connect](https://appstoreconnect.apple.com/) →
   **Users and Access** → **Integrations** → **App Store Connect API**.
2. Click **+** to generate a key and pick a role. **Admin** grants the full
   tool surface; **Developer** or **App Manager** restrict what the server can
   reach, and calls outside that role fail with a permissions error.
3. Download the `.p8` private key. **Apple lets you download it once.** Store
   it somewhere stable — moving or deleting it breaks the server.
4. Copy the **Key ID** (10 characters) and the **Issuer ID** (a UUID) shown on
   that page.

The server reads three environment variables, all required:

| Variable | Example | Meaning |
|----------|---------|---------|
| `ASC_ISSUER_ID` | `57246542-96fe-1a63-e053-0824d011072a` | Issuer ID (UUID) |
| `ASC_KEY_ID` | `2X9R4HXF34` | Key ID |
| `ASC_PRIVATE_KEY_PATH` | `/Users/you/.appstoreconnect/AuthKey_2X9R4HXF34.p8` | Absolute path to the `.p8` file |

Keep the key readable only by you:

```bash
chmod 600 /path/to/AuthKey_XXXXXXXXXX.p8
```

`config/config.sample.env` is a template if you prefer an env file.

## 3. Verify the configuration

Before touching any client, confirm the credentials resolve. `validate` checks
local configuration only and makes no API calls:

```bash
export ASC_ISSUER_ID="..."
export ASC_KEY_ID="..."
export ASC_PRIVATE_KEY_PATH="/absolute/path/AuthKey_XXXXXXXXXX.p8"

asc-mcp validate
```

```
Validating configuration...

[OK]   ASC_ISSUER_ID is set (57246542...)
[OK]   ASC_KEY_ID is set (2X9R4HXF34)
[OK]   ASC_PRIVATE_KEY_PATH exists (/Users/you/.appstoreconnect/AuthKey_2X9R4HXF34.p8)

[OK]   Configuration is valid
```

You can also confirm the server speaks MCP without any client at all:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"server/discover"}' | asc-mcp serve
```

A JSON response listing `supportedVersions` means the server is working, and
any client failure after this point is a configuration problem in the client.

## 4. Claude Desktop

Edit the configuration file:

- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

Create the file if it does not exist:

```json
{
  "mcpServers": {
    "asc-mcp": {
      "command": "/Users/you/go/bin/asc-mcp",
      "args": ["serve"],
      "env": {
        "ASC_ISSUER_ID": "57246542-96fe-1a63-e053-0824d011072a",
        "ASC_KEY_ID": "2X9R4HXF34",
        "ASC_PRIVATE_KEY_PATH": "/Users/you/.appstoreconnect/AuthKey_2X9R4HXF34.p8"
      }
    }
  }
}
```

Restart Claude Desktop completely — quit the application rather than closing
the window. The server appears in the tools menu once it connects.

Because Claude Desktop does not inherit your shell environment, the `env`
block is the only place these credentials come from. Both paths must be
absolute.

## 5. Claude Code (CLI)

Register the server with one command:

```bash
claude mcp add --scope user \
  --env ASC_ISSUER_ID=57246542-96fe-1a63-e053-0824d011072a \
  --env ASC_KEY_ID=2X9R4HXF34 \
  --env ASC_PRIVATE_KEY_PATH=/Users/you/.appstoreconnect/AuthKey_2X9R4HXF34.p8 \
  asc-mcp -- /Users/you/go/bin/asc-mcp serve
```

Everything after `--` is the command Claude Code runs verbatim; that separator
is what keeps `serve` from being parsed as an option to `claude`.

Manage servers with:

```bash
claude mcp list           # servers and their connection status
claude mcp get asc-mcp    # details for one server
claude mcp remove asc-mcp # remove it
```

### Choosing a scope

| Scope | Flag | Stored in | Use when |
|-------|------|-----------|----------|
| User | `--scope user` | `~/.claude.json` | You want the server in every project (recommended here) |
| Local | *(default)* | `~/.claude.json`, keyed by directory | Only this working directory should have it |
| Project | `--scope project` | `.mcp.json`, committed to the repo | The whole team should get it |

**Do not use project scope for this server.** Claude Code deliberately strips
environment variables whose names contain `KEY`, `TOKEN`, `SECRET`,
`PASSWORD`, or `AUTH` from project-scoped servers, so `ASC_KEY_ID` and
`ASC_PRIVATE_KEY_PATH` would never reach the process and it would fail to
start. That guard exists so credentials are not committed to a shared repo —
which is exactly what would happen otherwise, since these are personal
credentials tied to your App Store Connect account.

If you want the server committed for a team anyway, commit a `.mcp.json` that
points at the binary and let each developer supply credentials from their own
shell environment:

```json
{
  "mcpServers": {
    "asc-mcp": {
      "type": "stdio",
      "command": "/usr/local/bin/asc-mcp",
      "args": ["serve"]
    }
  }
}
```

Claude Code prompts each developer to approve a project-scoped server the
first time it loads.

## 6. Codex (app, CLI, and IDE extension)

Codex uses **TOML**, not JSON — a Claude config pasted here will not work. The
ChatGPT desktop app, the Codex CLI, and the IDE extension all read the same
file, so you configure this once.

Add to `~/.codex/config.toml` (or a project-scoped `.codex/config.toml` in a
trusted project):

```toml
[mcp_servers.asc-mcp]
command = "/Users/you/go/bin/asc-mcp"
args = ["serve"]

[mcp_servers.asc-mcp.env]
ASC_ISSUER_ID = "57246542-96fe-1a63-e053-0824d011072a"
ASC_KEY_ID = "2X9R4HXF34"
ASC_PRIVATE_KEY_PATH = "/Users/you/.appstoreconnect/AuthKey_2X9R4HXF34.p8"
```

The `[mcp_servers.asc-mcp.env]` table must come after the server's own keys.
In TOML a table header ends the previous table, so any `command` or `args`
line placed below it would be read as part of `env` instead.

### Codex CLI shortcut

The CLI can write that entry for you:

```bash
codex mcp add asc-mcp \
  --env ASC_ISSUER_ID=57246542-96fe-1a63-e053-0824d011072a \
  --env ASC_KEY_ID=2X9R4HXF34 \
  --env ASC_PRIVATE_KEY_PATH=/Users/you/.appstoreconnect/AuthKey_2X9R4HXF34.p8 \
  -- /Users/you/go/bin/asc-mcp serve
```

As with Claude Code, `--` separates Codex's own flags from the command it
launches.

### Passing credentials from your shell instead

To keep secrets out of the config file, name the variables Codex should
forward from your environment and omit the `env` table:

```toml
[mcp_servers.asc-mcp]
command = "/Users/you/go/bin/asc-mcp"
args = ["serve"]
env_vars = ["ASC_ISSUER_ID", "ASC_KEY_ID", "ASC_PRIVATE_KEY_PATH"]
```

This only works when the variables are actually exported in the environment
that launched Codex — reliable for the CLI, less so for the desktop app, which
may not see your shell profile.

## 7. Running over HTTP instead of stdio

Every client above spawns the server as a local subprocess, which is the right
default for a single user. Run the HTTP transport instead when the server
should be shared, run in a container, or live on another host.

```bash
asc-mcp serve-http --addr 127.0.0.1:8080 --auth-tokens "$ASC_MCP_AUTH_TOKENS"
```

The endpoint is `POST /mcp`, with `GET /healthz` for probes and `GET /metrics`
for Prometheus.

Useful flags:

| Flag | Default | Purpose |
|------|---------|---------|
| `--addr` | `:8080` | Bind address. Prefer `127.0.0.1:8080` unless remote access is intended |
| `--auth-tokens` | *(empty)* | Comma-separated Bearer tokens. Also read from `ASC_MCP_AUTH_TOKENS` |
| `--allowed-origins` | *(empty)* | Origin allowlist; defends browser clients against DNS rebinding |
| `--tls-cert` / `--tls-key` | *(empty)* | Terminate TLS in process (TLS 1.2 minimum) |
| `--log-format` | `json` | `json` or `text` |
| `--log-level` | `info` | `debug`, `info`, `warn`, `error` |

**An empty `--auth-tokens` leaves the endpoint unauthenticated.** Anyone who
can reach the port can act on your App Store Connect account. Set tokens for
anything beyond a loopback bind, and put it behind TLS — either with the flags
above or a terminating reverse proxy — before it crosses a network.

Point Claude Code at it with:

```bash
claude mcp add --scope user --transport http asc-mcp-http http://127.0.0.1:8080/mcp \
  --header "Authorization: Bearer YOUR_TOKEN"
```

And Codex with:

```toml
[mcp_servers.asc-mcp]
url = "http://127.0.0.1:8080/mcp"
bearer_token_env_var = "ASC_MCP_TOKEN"
```

Codex reads the token from the named environment variable rather than storing
it in the file.

## Troubleshooting

**The server does not appear, or shows as failed.**
Run `asc-mcp validate` in a terminal first. If that passes but the client still
fails, the client is not seeing what your shell sees — almost always a relative
command path or credentials that only exist in your shell profile. Use
absolute paths for both the binary and the `.p8` file, and set credentials in
the client's own `env` block.

**"ASC_ISSUER_ID environment variable is required"**
The client launched the binary without the environment. Check the `env` block
(Claude) or `[mcp_servers.asc-mcp.env]` table (Codex) is present and spelled
correctly.

**Credentials silently missing in Claude Code.**
You are likely on a project-scoped server. `ASC_KEY_ID` and
`ASC_PRIVATE_KEY_PATH` contain `KEY`, so Claude Code strips them from
`.mcp.json` servers. Re-add with `--scope user`. See
[section 5](#5-claude-code-cli).

**401 or 403 from App Store Connect.**
The JWT is being rejected or the key lacks permission. Confirm the Key ID
matches the `.p8` file, that the Issuer ID belongs to the same team, and that
the key's role covers what you are asking for — a Developer-role key cannot
touch pricing or user management, for example.

**Configuration changes have no effect.**
Restart the client. Claude Desktop in particular must be quit entirely, not
just closed, before it re-reads its config.

**Tools are missing or behave unexpectedly after an update.**
`asc-mcp tools` lists every registered tool. Apple removes API operations
between versions, and tools disappear with them; `CHANGELOG.md` records which
ones and what replaced them.
