#!/bin/bash
# TDLib compliance tests for temp_password_state

set -e

echo "Testing TempPasswordState vs TDLib..."

# Test 1: Default state
echo "Test 1: Default state should have no password"
cargo test -p rustgram-temp-password-state test_default --quiet

# Test 2: Serialization format
echo "Test 2: Store/parse format matches TDLib"
cargo test -p rustgram-temp-password-state test_store_parse --quiet

# Test 3: Time calculations
echo "Test 3: Remaining time calculation"
cargo test -p rustgram-temp-password-state test_remaining_time --quiet

# Test 4: API object conversion
echo "Test 4: API object conversion"
cargo test -p rustgram-temp-password-state test_to_api_object --quiet

echo "All compliance tests passed!"
