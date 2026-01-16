# asc-mcp

An MCP (Model Context Protocol) server for Apple App Store Connect.

## Features

**200 MCP tools** covering the complete App Store Connect API:

- **App Management**: List apps, get app details, view app versions
- **Build Management**: List and inspect builds, view processing status
- **App Store Versions**: Create, update, delete versions; submit for review
- **TestFlight**: Manage beta groups and testers, beta localizations, build beta details
- **Provisioning**: Manage bundle IDs, certificates, profiles, and devices
- **In-App Purchases**: Full CRUD for in-app purchases
- **Subscriptions**: Manage subscription groups, subscriptions, offer codes, win-back offers
- **Pricing & Availability**: Configure app pricing, territories, and availability
- **Age Ratings**: Manage age rating declarations and IDFA declarations
- **Localizations**: App info and version localizations
- **Customer Reviews**: Read and respond to customer reviews
- **App Events**: Create and manage in-app events
- **App Clips**: Manage default and advanced App Clip experiences
- **Screenshots & Previews**: Manage screenshot sets and app previews
- **Game Center**: Achievements and leaderboards
- **Xcode Cloud**: CI products, workflows, and build runs
- **Analytics**: Analytics report requests and data
- **Users & Roles**: Team member management and invitations
- **Sandbox Testing**: Manage sandbox tester accounts
- **Custom Product Pages**: A/B testing with custom product pages and experiments
- **Diagnostics**: Performance metrics, diagnostic signatures, and logs
- **Encryption**: Export compliance declarations
- **EULA & Categories**: License agreements and app categories
- **Alternative Distribution**: EU alternative distribution keys and marketplace search

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

### App Management (3 tools)

| Tool | Description |
|------|-------------|
| `list_apps` | List all apps in your account |
| `get_app` | Get detailed app information |
| `get_app_versions` | List all versions for an app |

### Build Management (2 tools)

| Tool | Description |
|------|-------------|
| `list_builds` | List builds (optionally filtered by app) |
| `get_build` | Get detailed build information |

### App Store Versions (9 tools)

| Tool | Description |
|------|-------------|
| `list_app_store_versions` | List all versions for an app |
| `get_app_store_version` | Get version details |
| `create_app_store_version` | Create a new app version |
| `update_app_store_version` | Update version metadata |
| `delete_app_store_version` | Delete a version |
| `submit_app_for_review` | Submit version for App Store review |
| `get_app_store_review_detail` | Get review submission details |
| `create_app_store_review_detail` | Create review submission |
| `update_app_store_review_detail` | Update review submission |

### TestFlight (7 tools)

| Tool | Description |
|------|-------------|
| `list_beta_groups` | List beta groups |
| `create_beta_group` | Create a new beta group |
| `delete_beta_group` | Delete a beta group |
| `list_beta_testers` | List beta testers |
| `invite_beta_tester` | Invite a new beta tester |
| `remove_beta_tester` | Remove a beta tester |
| `add_tester_to_group` | Add a tester to a beta group |

### Beta Review & Localizations (17 tools)

| Tool | Description |
|------|-------------|
| `list_beta_app_review_submissions` | List beta review submissions |
| `get_beta_app_review_submission` | Get beta review submission details |
| `create_beta_app_review_submission` | Submit build for beta review |
| `get_beta_license_agreement` | Get beta license agreement |
| `update_beta_license_agreement` | Update beta license agreement |
| `list_beta_app_localizations` | List beta app localizations |
| `get_beta_app_localization` | Get beta app localization |
| `create_beta_app_localization` | Create beta app localization |
| `update_beta_app_localization` | Update beta app localization |
| `delete_beta_app_localization` | Delete beta app localization |
| `list_beta_build_localizations` | List beta build localizations |
| `get_beta_build_localization` | Get beta build localization |
| `create_beta_build_localization` | Create beta build localization |
| `update_beta_build_localization` | Update beta build localization |
| `delete_beta_build_localization` | Delete beta build localization |
| `get_build_beta_detail` | Get build beta details |
| `update_build_beta_detail` | Update build beta details |

### Provisioning (6 tools)

| Tool | Description |
|------|-------------|
| `list_bundle_ids` | List registered bundle IDs |
| `get_bundle_id` | Get bundle ID details |
| `list_certificates` | List signing certificates |
| `list_profiles` | List provisioning profiles |
| `list_devices` | List registered devices |
| `register_device` | Register a new device |

### In-App Purchases (5 tools)

| Tool | Description |
|------|-------------|
| `list_in_app_purchases` | List in-app purchases for an app |
| `get_in_app_purchase` | Get in-app purchase details |
| `create_in_app_purchase` | Create a new in-app purchase |
| `update_in_app_purchase` | Update in-app purchase |
| `delete_in_app_purchase` | Delete in-app purchase |

### Subscriptions (4 tools)

| Tool | Description |
|------|-------------|
| `list_subscription_groups` | List subscription groups |
| `get_subscription_group` | Get subscription group details |
| `list_subscriptions` | List subscriptions in a group |
| `get_subscription` | Get subscription details |

### Promoted Purchases & Offers (14 tools)

| Tool | Description |
|------|-------------|
| `list_promoted_purchases` | List promoted in-app purchases |
| `get_promoted_purchase` | Get promoted purchase details |
| `create_promoted_purchase` | Create promoted purchase |
| `update_promoted_purchase` | Update promoted purchase |
| `delete_promoted_purchase` | Delete promoted purchase |
| `list_subscription_offer_codes` | List subscription offer codes |
| `get_subscription_offer_code` | Get offer code details |
| `create_subscription_offer_code` | Create offer code |
| `update_subscription_offer_code` | Update offer code |
| `list_win_back_offers` | List win-back offers |
| `get_win_back_offer` | Get win-back offer details |
| `create_win_back_offer` | Create win-back offer |
| `update_win_back_offer` | Update win-back offer |
| `delete_win_back_offer` | Delete win-back offer |

### Pricing & Availability (7 tools)

| Tool | Description |
|------|-------------|
| `get_app_price_schedule` | Get app price schedule |
| `list_app_price_points` | List price points for an app |
| `list_territories` | List available territories |
| `list_subscription_price_points` | List subscription price points |
| `get_app_availability` | Get app availability settings |
| `create_app_availability` | Create/update availability settings |
| `list_territory_availabilities` | List territory availability details |

### Age Ratings & IDFA (6 tools)

| Tool | Description |
|------|-------------|
| `get_age_rating_declaration` | Get age rating declaration |
| `update_age_rating_declaration` | Update age rating declaration |
| `get_idfa_declaration` | Get IDFA declaration |
| `create_idfa_declaration` | Create IDFA declaration |
| `update_idfa_declaration` | Update IDFA declaration |
| `delete_idfa_declaration` | Delete IDFA declaration |

### App Info Localizations (6 tools)

| Tool | Description |
|------|-------------|
| `get_app_infos` | Get app info for an app |
| `list_app_info_localizations` | List app info localizations |
| `get_app_info_localization` | Get app info localization |
| `create_app_info_localization` | Create app info localization |
| `update_app_info_localization` | Update app info localization |
| `delete_app_info_localization` | Delete app info localization |

### Version Localizations (5 tools)

| Tool | Description |
|------|-------------|
| `list_version_localizations` | List version localizations |
| `get_version_localization` | Get version localization |
| `create_version_localization` | Create version localization |
| `update_version_localization` | Update version localization |
| `delete_version_localization` | Delete version localization |

### Customer Reviews (4 tools)

| Tool | Description |
|------|-------------|
| `list_customer_reviews` | List customer reviews |
| `get_customer_review` | Get customer review details |
| `create_customer_review_response` | Respond to a review |
| `delete_customer_review_response` | Delete review response |

### App Events (5 tools)

| Tool | Description |
|------|-------------|
| `list_app_events` | List app events |
| `get_app_event` | Get app event details |
| `create_app_event` | Create an app event |
| `update_app_event` | Update app event |
| `delete_app_event` | Delete app event |

### Phased Release (4 tools)

| Tool | Description |
|------|-------------|
| `get_phased_release` | Get phased release status |
| `create_phased_release` | Create phased release |
| `update_phased_release` | Update phased release state |
| `delete_phased_release` | Delete phased release |

### Pre-Orders (4 tools)

| Tool | Description |
|------|-------------|
| `get_pre_order` | Get pre-order details |
| `create_pre_order` | Create pre-order |
| `update_pre_order` | Update pre-order |
| `delete_pre_order` | Delete pre-order |

### App Clips (6 tools)

| Tool | Description |
|------|-------------|
| `list_app_clips` | List App Clips |
| `get_app_clip` | Get App Clip details |
| `list_app_clip_default_experiences` | List default experiences |
| `get_app_clip_default_experience` | Get default experience |
| `list_app_clip_advanced_experiences` | List advanced experiences |
| `get_app_clip_advanced_experience` | Get advanced experience |

### Screenshots & Previews (8 tools)

| Tool | Description |
|------|-------------|
| `list_screenshot_sets` | List screenshot sets |
| `list_screenshots` | List screenshots in a set |
| `get_screenshot` | Get screenshot details |
| `delete_screenshot` | Delete screenshot |
| `list_preview_sets` | List app preview sets |
| `list_previews` | List previews in a set |
| `get_preview` | Get preview details |
| `delete_preview` | Delete preview |

### Custom Product Pages & Experiments (10 tools)

| Tool | Description |
|------|-------------|
| `list_app_custom_product_pages` | List custom product pages |
| `get_app_custom_product_page` | Get custom product page |
| `create_app_custom_product_page` | Create custom product page |
| `update_app_custom_product_page` | Update custom product page |
| `delete_app_custom_product_page` | Delete custom product page |
| `list_app_store_version_experiments` | List A/B test experiments |
| `get_app_store_version_experiment` | Get experiment details |
| `create_app_store_version_experiment` | Create experiment |
| `update_app_store_version_experiment` | Update experiment |
| `delete_app_store_version_experiment` | Delete experiment |

### Game Center (11 tools)

| Tool | Description |
|------|-------------|
| `get_game_center_detail` | Get Game Center details |
| `list_game_center_achievements` | List achievements |
| `get_game_center_achievement` | Get achievement details |
| `create_game_center_achievement` | Create achievement |
| `update_game_center_achievement` | Update achievement |
| `delete_game_center_achievement` | Delete achievement |
| `list_game_center_leaderboards` | List leaderboards |
| `get_game_center_leaderboard` | Get leaderboard details |
| `create_game_center_leaderboard` | Create leaderboard |
| `update_game_center_leaderboard` | Update leaderboard |
| `delete_game_center_leaderboard` | Delete leaderboard |

### Xcode Cloud (8 tools)

| Tool | Description |
|------|-------------|
| `list_ci_products` | List CI products |
| `get_ci_product` | Get CI product details |
| `list_ci_workflows` | List CI workflows |
| `get_ci_workflow` | Get CI workflow details |
| `list_ci_build_runs` | List CI build runs |
| `get_ci_build_run` | Get CI build run details |
| `start_ci_build_run` | Start a new build run |
| `cancel_ci_build_run` | Cancel a build run |

### Analytics (7 tools)

| Tool | Description |
|------|-------------|
| `list_analytics_report_requests` | List analytics requests |
| `get_analytics_report_request` | Get analytics request |
| `create_analytics_report_request` | Create analytics request |
| `delete_analytics_report_request` | Delete analytics request |
| `list_analytics_reports` | List analytics reports |
| `list_analytics_report_instances` | List report instances |
| `list_analytics_report_segments` | List report segments |

### Diagnostics & Metrics (10 tools)

| Tool | Description |
|------|-------------|
| `list_perf_power_metrics` | List performance/power metrics |
| `list_diagnostic_signatures` | List diagnostic signatures |
| `list_diagnostic_logs` | List diagnostic logs |
| `list_app_store_review_attachments` | List review attachments |
| `get_app_store_review_attachment` | Get review attachment |
| `create_app_store_review_attachment` | Create review attachment |
| `delete_app_store_review_attachment` | Delete review attachment |
| `get_routing_app_coverage` | Get routing app coverage |
| `create_routing_app_coverage` | Create routing app coverage |
| `delete_routing_app_coverage` | Delete routing app coverage |

### Users & Roles (8 tools)

| Tool | Description |
|------|-------------|
| `list_users` | List team members |
| `get_user` | Get user details |
| `update_user` | Update user roles |
| `delete_user` | Remove user from team |
| `list_user_invitations` | List pending invitations |
| `get_user_invitation` | Get invitation details |
| `create_user_invitation` | Invite new user |
| `delete_user_invitation` | Cancel invitation |

### Sandbox Testers (4 tools)

| Tool | Description |
|------|-------------|
| `list_sandbox_testers` | List sandbox testers |
| `create_sandbox_tester` | Create sandbox tester |
| `update_sandbox_tester` | Update sandbox tester |
| `delete_sandbox_tester` | Delete sandbox tester |

### Encryption Declarations (4 tools)

| Tool | Description |
|------|-------------|
| `list_encryption_declarations` | List encryption declarations |
| `get_encryption_declaration` | Get declaration details |
| `create_encryption_declaration` | Create declaration |
| `assign_build_to_encryption_declaration` | Assign build to declaration |

### Reports (2 tools)

| Tool | Description |
|------|-------------|
| `get_sales_report` | Get sales and trends reports |
| `get_finance_report` | Get financial reports |

### EULA (4 tools)

| Tool | Description |
|------|-------------|
| `get_end_user_license_agreement` | Get EULA |
| `create_end_user_license_agreement` | Create EULA |
| `update_end_user_license_agreement` | Update EULA |
| `delete_end_user_license_agreement` | Delete EULA |

### App Categories (2 tools)

| Tool | Description |
|------|-------------|
| `list_app_categories` | List app categories |
| `get_app_category` | Get category details |

### Alternative Distribution (4 tools)

| Tool | Description |
|------|-------------|
| `list_alternative_distribution_keys` | List distribution keys |
| `get_alternative_distribution_key` | Get distribution key |
| `create_alternative_distribution_key` | Create distribution key |
| `delete_alternative_distribution_key` | Delete distribution key |

### Marketplace Search (4 tools)

| Tool | Description |
|------|-------------|
| `get_marketplace_search_detail` | Get marketplace search detail |
| `create_marketplace_search_detail` | Create marketplace search detail |
| `update_marketplace_search_detail` | Update marketplace search detail |
| `delete_marketplace_search_detail` | Delete marketplace search detail |

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
