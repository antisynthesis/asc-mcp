# ADR-0007: MCP Tool Organization

## Status

Accepted

## Context

The MCP server exposes 18 tools for interacting with App Store Connect API. These tools need to be:

- Organized logically for maintainability
- Discoverable by users and AI assistants
- Consistent in their interface patterns
- Easy to extend with new tools

## Decision

We will organize tools into four categories based on App Store Connect functionality:

1. **App Management** (`tools/apps.go`):
   - `list_apps` - List all apps
   - `get_app` - Get app details
   - `get_app_versions` - Get version history

2. **Builds** (`tools/builds.go`):
   - `list_builds` - List builds
   - `get_build` - Get build details

3. **TestFlight** (`tools/testflight.go`):
   - `list_beta_groups` - List beta groups
   - `create_beta_group` - Create beta group
   - `delete_beta_group` - Delete beta group
   - `list_beta_testers` - List testers
   - `invite_beta_tester` - Invite tester
   - `remove_beta_tester` - Remove tester
   - `add_tester_to_group` - Add tester to group

4. **Provisioning** (`tools/provisioning.go`):
   - `list_bundle_ids` - List bundle IDs
   - `get_bundle_id` - Get bundle ID details
   - `list_certificates` - List certificates
   - `list_profiles` - List profiles
   - `list_devices` - List devices
   - `register_device` - Register device

Tool Registration Pattern:
- Each file registers its tools via `RegisterTool()`
- Central registry in `tools/registry.go`
- Tools provide JSON Schema for input validation
- Consistent naming: `verb_noun` (e.g., `list_apps`, `get_build`)

## Consequences

### Positive

- Clear organization by domain
- Easy to locate and modify tools
- Consistent naming helps AI discovery
- JSON Schema enables input validation
- Registry pattern enables dynamic listing

### Negative

- Adding tools requires updating multiple places
- Some tools span categories (ambiguity)

### Mitigations

- Registration happens at package init
- Category grouping is flexible
- Documentation clarifies tool purposes
