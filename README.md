# asc-mcp

An MCP (Model Context Protocol) server for Apple App Store Connect.

## Features

**356 MCP tools** covering the major capability areas of the App Store
Connect API (tracking API version 4.4.1), including the three-step asset
upload flow for screenshots, previews, and review attachments:

- **App Management**: List apps, get app details, view app versions
- **Build Management**: List and inspect builds, view processing status
- **App Store Versions**: Create, update, delete versions; submit for review
- **TestFlight**: Manage beta groups and testers, beta localizations, build beta details
- **Provisioning**: Manage bundle IDs, certificates, profiles, and devices
- **In-App Purchases**: Full CRUD for in-app purchases
- **Subscriptions**: Manage subscription groups, subscriptions, offer codes, win-back offers
- **Pricing & Availability**: Configure app pricing, territories, and availability
- **Age Ratings**: Manage age rating declarations, including per-territory ratings
- **Localizations**: App info and version localizations
- **Customer Reviews**: Read and respond to customer reviews
- **App Events**: Create and manage in-app events
- **App Clips**: Manage default and advanced App Clip experiences
- **Screenshots & Previews**: Manage screenshot sets and app previews
- **Game Center**: Achievements, leaderboards, leaderboard sets, activities,
  challenges, and server-side player submissions
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

### Commerce Versioning (29 tools)

In-app purchases, subscriptions and subscription groups are versioned
(App Store Connect API 4.4.1). Create a version, edit its localizations
and images, then attach the version to a review submission with
`add_review_submission_item`.

| Tool | Description |
|------|-------------|
| `create_in_app_purchase_version` | Create an in-app purchase version |
| `get_in_app_purchase_version` | Get an in-app purchase version |
| `list_in_app_purchase_versions` | List versions of an in-app purchase |
| `list_in_app_purchase_version_localizations` | List a version's localizations |
| `create_in_app_purchase_localization` | Add a localization to a version |
| `update_in_app_purchase_localization` | Update a localization |
| `delete_in_app_purchase_localization` | Delete a localization |
| `list_in_app_purchase_version_images` | List a version's promotional images |
| `create_in_app_purchase_image` | Reserve an image upload on a version |
| `update_in_app_purchase_image` | Commit an image upload |
| `delete_in_app_purchase_image` | Delete a promotional image |
| `create_subscription_version` | Create a subscription version |
| `get_subscription_version` | Get a subscription version |
| `list_subscription_versions` | List versions of a subscription |
| `list_subscription_version_localizations` | List a version's localizations |
| `create_subscription_localization` | Add a localization to a version |
| `update_subscription_localization` | Update a localization |
| `delete_subscription_localization` | Delete a localization |
| `list_subscription_version_images` | List a version's promotional images |
| `create_subscription_image` | Reserve an image upload on a version |
| `update_subscription_image` | Commit an image upload |
| `delete_subscription_image` | Delete a promotional image |
| `create_subscription_group_version` | Create a subscription group version |
| `get_subscription_group_version` | Get a subscription group version |
| `list_subscription_group_versions` | List versions of a subscription group |
| `list_subscription_group_version_localizations` | List a version's localizations |
| `create_subscription_group_localization` | Add a localization to a version |
| `update_subscription_group_localization` | Update a localization |
| `delete_subscription_group_localization` | Delete a localization |

### In-App Purchase Offer Codes (12 tools)

| Tool | Description |
|------|-------------|
| `list_in_app_purchase_offer_codes` | List an in-app purchase's offer codes |
| `get_in_app_purchase_offer_code` | Get offer code details |
| `create_in_app_purchase_offer_code` | Create an offer code with per-territory prices |
| `update_in_app_purchase_offer_code` | Activate or deactivate an offer code |
| `list_in_app_purchase_offer_code_prices` | List an offer code's prices |
| `list_in_app_purchase_offer_code_custom_codes` | List custom code batches |
| `create_in_app_purchase_offer_code_custom_code` | Issue a custom (memorable) code |
| `update_in_app_purchase_offer_code_custom_code` | Activate or deactivate a custom code |
| `list_in_app_purchase_offer_code_one_time_use_codes` | List one-time-use code batches |
| `create_in_app_purchase_offer_code_one_time_use_code` | Generate a one-time-use code batch |
| `update_in_app_purchase_offer_code_one_time_use_code` | Activate or deactivate a batch |
| `get_in_app_purchase_offer_code_one_time_use_code_values` | Download generated codes as CSV |

### Commerce Pricing & Availability (18 tools)

| Tool | Description |
|------|-------------|
| `list_in_app_purchase_price_points` | List an in-app purchase's price points |
| `get_in_app_purchase_price_schedule` | Get an in-app purchase's price schedule |
| `set_in_app_purchase_price_schedule` | Replace an in-app purchase's prices |
| `list_in_app_purchase_price_schedule_prices` | List manual or automatic scheduled prices |
| `get_in_app_purchase_availability` | Get in-app purchase territory availability |
| `set_in_app_purchase_availability` | Set in-app purchase territory availability |
| `list_in_app_purchase_available_territories` | List covered territories |
| `list_subscription_plan_availabilities` | List a subscription's plan availabilities |
| `get_subscription_plan_availability` | Get a plan availability |
| `create_subscription_plan_availability` | Configure a plan's territory availability |
| `update_subscription_plan_availability` | Update a plan's territory availability |
| `list_subscription_plan_available_territories` | List covered territories |
| `get_subscription_price_point` | Get a subscription price point |
| `list_subscription_price_point_equalizations` | List equivalent price points |
| `list_subscription_price_point_adjusted_equalizations` | List pre-paid-adjusted equalizations |
| `set_app_price_schedule` | Replace an app's price schedule |
| `get_app_price_point` | Get an app price point |
| `list_app_price_point_equalizations` | List equivalent app price points |

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

### Age Ratings (3 tools)

| Tool | Description |
|------|-------------|
| `get_age_rating_declaration` | Get age rating declaration |
| `update_age_rating_declaration` | Update age rating declaration |
| `list_territory_age_ratings` | List age ratings per territory |

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

### Pre-Orders (1 tool)

Apple removed the `appPreOrders` resource; ending a pre-order is now an
availability operation.

| Tool | Description |
|------|-------------|
| `end_app_availability_pre_order` | End an app's pre-order period |

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

### Game Center Leaderboard Sets (20 tools)

| Tool | Description |
|------|-------------|
| `list_game_center_leaderboard_sets` | List leaderboard sets |
| `get_game_center_leaderboard_set` | Get leaderboard set details |
| `create_game_center_leaderboard_set` | Create leaderboard set |
| `update_game_center_leaderboard_set` | Update leaderboard set |
| `delete_game_center_leaderboard_set` | Delete leaderboard set |
| `list_game_center_leaderboard_set_members` | List member leaderboards |
| `add_game_center_leaderboard_set_members` | Add leaderboards to a set |
| `remove_game_center_leaderboard_set_members` | Remove leaderboards from a set |
| `list_game_center_leaderboard_set_versions` | List leaderboard set versions |
| `get_game_center_leaderboard_set_version` | Get leaderboard set version |
| `create_game_center_leaderboard_set_version` | Open a new version |
| `list_game_center_leaderboard_set_localizations` | List localizations |
| `create_game_center_leaderboard_set_localization` | Create localization |
| `update_game_center_leaderboard_set_localization` | Update localization |
| `delete_game_center_leaderboard_set_localization` | Delete localization |
| `upload_game_center_leaderboard_set_image` | Upload localization image |
| `list_game_center_leaderboard_set_member_localizations` | List per-set leaderboard names |
| `create_game_center_leaderboard_set_member_localization` | Name a leaderboard within a set |
| `update_game_center_leaderboard_set_member_localization` | Update a per-set name |
| `delete_game_center_leaderboard_set_member_localization` | Delete a per-set name |

### Game Center Activities (14 tools)

| Tool | Description |
|------|-------------|
| `list_game_center_activities` | List activities |
| `get_game_center_activity` | Get activity details |
| `create_game_center_activity` | Create activity with its initial version |
| `update_game_center_activity` | Update activity |
| `delete_game_center_activity` | Delete activity |
| `list_game_center_activity_versions` | List activity versions |
| `get_game_center_activity_version` | Get activity version |
| `create_game_center_activity_version` | Open a new version |
| `update_game_center_activity_version` | Update a version's fallback URL |
| `list_game_center_activity_localizations` | List localizations |
| `create_game_center_activity_localization` | Create localization |
| `update_game_center_activity_localization` | Update localization |
| `delete_game_center_activity_localization` | Delete localization |
| `upload_game_center_activity_image` | Upload activity image |

### Game Center Challenges (13 tools)

| Tool | Description |
|------|-------------|
| `list_game_center_challenges` | List challenges |
| `get_game_center_challenge` | Get challenge details |
| `create_game_center_challenge` | Create challenge with its initial version |
| `update_game_center_challenge` | Update challenge |
| `delete_game_center_challenge` | Delete challenge |
| `list_game_center_challenge_versions` | List challenge versions |
| `get_game_center_challenge_version` | Get challenge version |
| `create_game_center_challenge_version` | Open a new version |
| `list_game_center_challenge_localizations` | List localizations |
| `create_game_center_challenge_localization` | Create localization |
| `update_game_center_challenge_localization` | Update localization |
| `delete_game_center_challenge_localization` | Delete localization |
| `upload_game_center_challenge_image` | Upload challenge image |

### Game Center Player Submissions (2 tools)

| Tool | Description |
|------|-------------|
| `submit_game_center_player_achievement` | Submit achievement progress for a player |
| `submit_game_center_leaderboard_entry` | Submit a leaderboard score for a player |

Game Center content reaches App Review through `add_review_submission_item`:
attach the achievement, leaderboard, leaderboard set, activity, or challenge
*version* to a draft review submission.

### Xcode Cloud (7 tools)

| Tool | Description |
|------|-------------|
| `list_ci_products` | List CI products |
| `get_ci_product` | Get CI product details |
| `list_ci_workflows` | List CI workflows |
| `get_ci_workflow` | Get CI workflow details |
| `list_ci_build_runs` | List CI build runs |
| `get_ci_build_run` | Get CI build run details |
| `start_ci_build_run` | Start a new build run |

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

### Sandbox Testers (3 tools)

Sandbox testers are created and deleted in App Store Connect; the API
exposes only reads, updates, and purchase-history resets.

| Tool | Description |
|------|-------------|
| `list_sandbox_testers` | List sandbox testers |
| `update_sandbox_tester` | Update sandbox tester |
| `clear_sandbox_tester_purchase_history` | Clear a tester's purchase history |

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

### Review Submissions (6 tools)

App Store versions, custom product pages, experiments, app events, Game
Center content, and commerce versions all reach App Review through this
flow.

| Tool | Description |
|------|-------------|
| `list_review_submissions` | List review submissions for an app |
| `get_review_submission` | Get a submission and its attached items |
| `create_review_submission` | Open a review submission |
| `add_review_submission_item` | Attach an item to a submission |
| `submit_review_submission` | Submit for review |
| `cancel_review_submission` | Cancel a submission |

### Webhooks (8 tools)

| Tool | Description |
|------|-------------|
| `list_webhooks` | List webhooks for an app |
| `get_webhook` | Get webhook details |
| `create_webhook` | Create a webhook |
| `update_webhook` | Update a webhook |
| `delete_webhook` | Delete a webhook |
| `ping_webhook` | Send a test delivery |
| `list_webhook_deliveries` | List deliveries for a webhook |
| `redeliver_webhook_delivery` | Redeliver a past delivery |

### Build Uploads (6 tools)

| Tool | Description |
|------|-------------|
| `list_build_uploads` | List build uploads |
| `get_build_upload` | Get build upload details |
| `start_build_upload` | Reserve a new build upload |
| `list_build_upload_files` | List files in a build upload |
| `upload_build_file` | Upload and commit a build file |
| `delete_build_upload` | Delete a build upload |

### TestFlight Beta Feedback (8 tools)

| Tool | Description |
|------|-------------|
| `list_beta_feedback_crashes` | List crash feedback from testers |
| `get_beta_feedback_crash` | Get crash feedback details |
| `get_beta_feedback_crash_log` | Download a crash log |
| `delete_beta_feedback_crash` | Delete crash feedback |
| `list_beta_feedback_screenshots` | List screenshot feedback |
| `get_beta_feedback_screenshot` | Get screenshot feedback details |
| `delete_beta_feedback_screenshot` | Delete screenshot feedback |
| `get_beta_build_usage_metrics` | Get beta build usage metrics |

### Background Assets (10 tools)

| Tool | Description |
|------|-------------|
| `list_background_assets` | List Apple-hosted background assets |
| `get_background_asset` | Get background asset details |
| `create_background_asset` | Create a background asset |
| `update_background_asset` | Update a background asset |
| `list_background_asset_versions` | List asset versions |
| `get_background_asset_version` | Get a version, including state details |
| `create_background_asset_version` | Create a version |
| `list_background_asset_upload_files` | List upload files for a version |
| `upload_background_asset_file` | Upload and commit an asset pack |
| `get_background_asset_version_release` | Get a version's release state |

### Accessibility Declarations (5 tools)

| Tool | Description |
|------|-------------|
| `list_accessibility_declarations` | List declarations per device family |
| `get_accessibility_declaration` | Get declaration details |
| `create_accessibility_declaration` | Create a declaration |
| `update_accessibility_declaration` | Update a declaration |
| `delete_accessibility_declaration` | Delete a declaration |

### App Tags (3 tools)

| Tool | Description |
|------|-------------|
| `list_app_tags` | List Apple-created tags for an app |
| `list_app_tag_territories` | List territories a tag applies to |
| `update_app_tag` | Opt in or out of a tag |

### Android to iOS App Mapping (5 tools)

| Tool | Description |
|------|-------------|
| `list_android_to_ios_app_mappings` | List Android app mappings |
| `get_android_to_ios_app_mapping` | Get a mapping |
| `create_android_to_ios_app_mapping` | Map an Android app to this app |
| `update_android_to_ios_app_mapping` | Update a mapping |
| `delete_android_to_ios_app_mapping` | Delete a mapping |

### Asset Uploads (3 tools)

| Tool | Description |
|------|-------------|
| `upload_app_screenshot` | Upload a screenshot |
| `upload_app_preview` | Upload an app preview |
| `upload_review_attachment` | Upload a review attachment |

### Other (2 tools)

| Tool | Description |
|------|-------------|
| `list_customer_review_summarizations` | Read customer review summaries |
| `get_alternative_distribution_package` | Get a version's distribution package |

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

The MCP protocol layer, JWT/ES256 signing, and HTTP client are implemented
using only the Go standard library. The CLI is built on
[`spf13/cobra`](https://github.com/spf13/cobra) (see ADR-0004). No other
runtime dependencies are required.

The server is dual-era: it speaks the stateless MCP protocol revision
`2026-07-28` (per-request `_meta` version/capabilities, `server/discover`,
no sessions) and still serves handshake-based clients, negotiating
`2025-11-25`, `2025-06-18`, `2025-03-26`, or `2024-11-05` via `initialize`.
Each request picks its era: a `_meta` `io.modelcontextprotocol/protocolVersion`
key (or a `server/discover` call) selects the modern path; everything else
takes the legacy path unchanged.

Two transports are supported, sharing the same dispatcher and tool registry:

- `asc-mcp serve` — stdio. JSON-RPC messages on stdin/stdout. Use this when
  the server is spawned by a desktop client such as Claude Desktop.
- `asc-mcp serve-http --addr :8080` — Streamable HTTP. `POST /mcp` for
  JSON-RPC submissions, `GET /healthz` for the K8s probe, `GET /metrics`
  for Prometheus scraping. Modern (`2026-07-28`) requests are sessionless
  and must carry the `Mcp-Method` header (plus `Mcp-Name` on `tools/call`);
  legacy clients get an `Mcp-Session-Id` on initialize, echo it on every
  subsequent request, and may `DELETE /mcp` to end the session. Useful
  flags:
  - `--auth-tokens` (or `ASC_MCP_AUTH_TOKENS` env): comma-separated
    Bearer tokens. Required for any non-trusted-network deployment.
  - `--allowed-origins`: comma-separated Origin allowlist (DNS rebinding
    defense for browser clients).
  - `--tls-cert` / `--tls-key`: PEM cert+key to terminate TLS in
    process (TLS 1.2 minimum). Omit both to listen plain HTTP behind a
    TLS-terminating reverse proxy.
  - `--log-format` (json|text), `--log-level` (debug|info|warn|error).

Asset uploads (screenshots, previews, review attachments) use the
three-step App Store Connect handshake (reserve → chunked upload →
commit) under the hood. Tools accept either a local `file_path` or a
base64-encoded `file_data_base64` so they work both from stdio and over
the HTTP transport.

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
