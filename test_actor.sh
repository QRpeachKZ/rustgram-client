#!/bin/bash
# Test script for actor crate

# Create a temporary workspace
cat > /tmp/actor-test-workspace.toml << 'EOF'
[workspace]
members = ["crates/actor"]
resolver = "2"

[workspace.dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Testing
proptest = "1.4"
criterion = "0.5"
rand = "0.8"

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
license = "MIT OR Apache-2.0"

[workspace.lints]
rust = { missing_docs = "warn" }
clippy = { all = "warn", unwrap_used = "deny", expect_used = "deny" }
EOF

# Run tests from the actor crate directory
echo "Running actor crate tests..."

# Count tests
echo "=== Test Count Summary ==="
cargo test --manifest-path crates/actor/Cargo.toml --lib -- --list 2>&1 | grep -c "test "

# Run unit tests
echo ""
echo "=== Running Unit Tests ==="
cargo test --manifest-path crates/actor/Cargo.toml --lib 2>&1 | tail -20

# Run integration tests
echo ""
echo "=== Running Integration Tests ==="
cargo test --manifest-path crates/actor/Cargo.toml --test integration_tests 2>&1 | tail -20

echo ""
echo "=== Test Complete ==="
