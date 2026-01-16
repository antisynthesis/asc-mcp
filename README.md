# asc-mcp

An MCP (Model Context Protocol) server for Apple App Store Connect.

## Features

- **App Management**: List apps, get app details, view app versions
- **Build Management**: List and inspect builds, view processing status
- **TestFlight**: Manage beta groups and testers, invite testers, configure public links
- **Provisioning**: Manage bundle IDs, certificates, profiles, and devices

## Prerequisites

- Go 1.23 or later
- App Store Connect API credentials

## Getting App Store Connect API Credentials

1. Go to [App Store Connect](https://appstoreconnect.apple.com/)
2. Navigate to **Users and Access** > **Integrations** > **App Store Connect API**
3. Click **+** to generate a new key
4. Select the appropriate role:
   - **Admin**: Full access to all features
   - **Developer**: Access to app metadata, builds, and TestFlight
   - **App Manager**: Limited access to specific apps
5. Download the `.p8` private key file (this can only be downloaded once)
6. Note the **Key ID** and **Issuer ID** displayed on the page

## Configuration

Set the following environment variables:

```bash
export ASC_ISSUER_ID="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
export ASC_KEY_ID="XXXXXXXXXX"
export ASC_PRIVATE_KEY_PATH="/path/to/AuthKey_XXXXXXXXXX.p8"
```

See `config/config.sample.env` for a template.

## Building

```bash
make build
```

The binary will be placed in `bin/asc-mcp`.

## Usage with Claude

### Claude Desktop Configuration

Add the following to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "asc-mcp": {
      "command": "/path/to/asc-mcp",
      "env": {
        "ASC_ISSUER_ID": "your-issuer-id",
        "ASC_KEY_ID": "your-key-id",
        "ASC_PRIVATE_KEY_PATH": "/path/to/AuthKey.p8"
      }
    }
  }
}
```

### Claude Code Configuration

Add to your project's `.mcp.json`:

```json
{
  "mcpServers": {
    "asc-mcp": {
      "command": "/path/to/asc-mcp",
      "env": {
        "ASC_ISSUER_ID": "your-issuer-id",
        "ASC_KEY_ID": "your-key-id",
        "ASC_PRIVATE_KEY_PATH": "/path/to/AuthKey.p8"
      }
    }
  }
}
```

## Available Tools

### App Management

| Tool | Description |
|------|-------------|
| `list_apps` | List all apps in your account |
| `get_app` | Get detailed app information |
| `get_app_versions` | List all versions for an app |

### Build Management

| Tool | Description |
|------|-------------|
| `list_builds` | List builds (optionally filtered by app) |
| `get_build` | Get detailed build information |

### TestFlight

| Tool | Description |
|------|-------------|
| `list_beta_groups` | List beta groups |
| `create_beta_group` | Create a new beta group |
| `delete_beta_group` | Delete a beta group |
| `list_beta_testers` | List beta testers |
| `invite_beta_tester` | Invite a new beta tester |
| `remove_beta_tester` | Remove a beta tester |
| `add_tester_to_group` | Add a tester to a beta group |

### Provisioning

| Tool | Description |
|------|-------------|
| `list_bundle_ids` | List registered bundle IDs |
| `get_bundle_id` | Get bundle ID details |
| `list_certificates` | List signing certificates |
| `list_profiles` | List provisioning profiles |
| `list_devices` | List registered devices |
| `register_device` | Register a new device |

## Development

### Running Tests

```bash
make test
```

### Code Formatting

```bash
make fmt
```

### Linting

```bash
make lint
```

## Architecture

This project uses only the Go standard library. No external dependencies are required.

```
asc-mcp/
├── cmd/asc-mcp/          # Application entry point
├── internal/asc/
│   ├── api/              # App Store Connect API client
│   ├── config/           # Configuration management
│   ├── server/           # MCP server implementation
│   └── tools/            # Tool implementations
├── config/               # Configuration templates
├── script/               # Build and test scripts
└── doc/                  # Documentation
```

## License

MIT License
