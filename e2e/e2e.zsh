#!/usr/bin/env zsh
set -euo pipefail

# End-to-end tests for asc-mcp
# This script tests the MCP server by sending JSON-RPC requests and validating responses

SCRIPT_DIR="${0:A:h}"
PROJECT_ROOT="${SCRIPT_DIR:h}"
BINARY="${PROJECT_ROOT}/bin/asc-mcp"

# Colors for output (using tput for portability)
if [[ -t 1 ]]; then
    RED=$(tput setaf 1)
    GREEN=$(tput setaf 2)
    YELLOW=$(tput setaf 3)
    RESET=$(tput sgr0)
else
    RED=""
    GREEN=""
    YELLOW=""
    RESET=""
fi

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Logging functions
log_info() {
    echo "${YELLOW}[INFO]${RESET} $1"
}

log_pass() {
    echo "${GREEN}[PASS]${RESET} $1"
    : $(( TESTS_PASSED += 1 ))
}

log_fail() {
    echo "${RED}[FAIL]${RESET} $1"
    : $(( TESTS_FAILED += 1 ))
}

# Cleanup function
cleanup() {
    log_info "Cleaning up..."
    # Kill any background processes
    jobs -p | xargs -r kill 2>/dev/null || true
}

trap cleanup EXIT

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    if [[ ! -x "${BINARY}" ]]; then
        log_info "Binary not found, building..."
        (cd "${PROJECT_ROOT}" && ./script/build.zsh)
    fi

    if [[ ! -x "${BINARY}" ]]; then
        log_fail "Failed to build binary"
        exit 1
    fi

    log_pass "Prerequisites check"
}

# Test: Version command
test_version_command() {
    : $(( TESTS_RUN += 1 ))
    log_info "Testing: version command"

    local output
    output=$("${BINARY}" version 2>&1)

    if [[ "${output}" == *"asc-mcp version"* ]]; then
        log_pass "Version command returns version info"
    else
        log_fail "Version command failed: ${output}"
    fi
}

# Test: Tools command
test_tools_command() {
    : $(( TESTS_RUN += 1 ))
    log_info "Testing: tools command"

    local output
    output=$("${BINARY}" tools 2>&1)

    if [[ "${output}" == *"Available MCP Tools"* ]] && [[ "${output}" == *"list_apps"* ]]; then
        log_pass "Tools command lists available tools"
    else
        log_fail "Tools command failed: ${output}"
    fi
}

# Test: Validate command (without credentials)
test_validate_command_no_creds() {
    : $(( TESTS_RUN += 1 ))
    log_info "Testing: validate command without credentials"

    # Unset credentials for this test
    local old_issuer="${ASC_ISSUER_ID:-}"
    local old_key="${ASC_KEY_ID:-}"
    local old_path="${ASC_PRIVATE_KEY_PATH:-}"

    unset ASC_ISSUER_ID ASC_KEY_ID ASC_PRIVATE_KEY_PATH

    local output
    local exit_code=0
    output=$("${BINARY}" validate 2>&1) || exit_code=$?

    # Restore credentials
    [[ -n "${old_issuer}" ]] && export ASC_ISSUER_ID="${old_issuer}"
    [[ -n "${old_key}" ]] && export ASC_KEY_ID="${old_key}"
    [[ -n "${old_path}" ]] && export ASC_PRIVATE_KEY_PATH="${old_path}"

    if [[ ${exit_code} -ne 0 ]] && [[ "${output}" == *"FAIL"* ]]; then
        log_pass "Validate command correctly fails without credentials"
    else
        log_fail "Validate command should fail without credentials"
    fi
}

# Test: Help command
test_help_command() {
    : $(( TESTS_RUN += 1 ))
    log_info "Testing: help command"

    local output
    output=$("${BINARY}" --help 2>&1)

    if [[ "${output}" == *"App Store Connect"* ]] && [[ "${output}" == *"serve"* ]]; then
        log_pass "Help command shows usage information"
    else
        log_fail "Help command failed: ${output}"
    fi
}

# Test: Serve help
test_serve_help() {
    : $(( TESTS_RUN += 1 ))
    log_info "Testing: serve help"

    local output
    output=$("${BINARY}" serve --help 2>&1)

    if [[ "${output}" == *"Start the MCP server"* ]] && [[ "${output}" == *"ASC_ISSUER_ID"* ]]; then
        log_pass "Serve help shows configuration info"
    else
        log_fail "Serve help failed: ${output}"
    fi
}

# Test: MCP initialize (requires running server)
test_mcp_initialize() {
    : $(( TESTS_RUN += 1 ))
    log_info "Testing: MCP initialize request (mock)"

    # Skip if no credentials are configured
    if [[ -z "${ASC_ISSUER_ID:-}" ]] || [[ -z "${ASC_KEY_ID:-}" ]] || [[ -z "${ASC_PRIVATE_KEY_PATH:-}" ]]; then
        log_info "Skipping MCP server test - credentials not configured"
        return 0
    fi

    # Create a temporary file for the response
    local response_file
    response_file=$(mktemp)

    # Send initialize request to the server
    local request='{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"e2e-test","version":"1.0.0"}}}'

    # Start server and send request
    echo "${request}" | timeout 5 "${BINARY}" serve > "${response_file}" 2>/dev/null &
    local pid=$!

    sleep 2
    kill ${pid} 2>/dev/null || true

    local response
    response=$(cat "${response_file}")
    rm -f "${response_file}"

    if [[ "${response}" == *'"protocolVersion"'* ]] && [[ "${response}" == *'"serverInfo"'* ]]; then
        log_pass "MCP initialize returns valid response"
    else
        log_fail "MCP initialize failed: ${response}"
    fi
}

# Main
main() {
    echo "========================================"
    echo "asc-mcp End-to-End Tests"
    echo "========================================"
    echo ""

    check_prerequisites

    echo ""
    echo "Running tests..."
    echo ""

    test_version_command
    test_tools_command
    test_validate_command_no_creds
    test_help_command
    test_serve_help
    test_mcp_initialize

    echo ""
    echo "========================================"
    echo "Test Results"
    echo "========================================"
    echo "Tests run:    ${TESTS_RUN}"
    echo "Tests passed: ${GREEN}${TESTS_PASSED}${RESET}"
    echo "Tests failed: ${RED}${TESTS_FAILED}${RESET}"
    echo ""

    if [[ ${TESTS_FAILED} -gt 0 ]]; then
        echo "${RED}Some tests failed${RESET}"
        exit 1
    else
        echo "${GREEN}All tests passed${RESET}"
        exit 0
    fi
}

main "$@"
