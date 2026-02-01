# Rustgram Client

Native Rust Telegram client based on TDLib/MTProto protocol.

## Project Overview

**WHAT**: Telegram client implementation in Rust with 400+ crates using actor-based architecture.

**WHY**: Type-safe, memory-safe alternative to TDLib with modern async/await patterns.

**STRUCTURE**:
- `crates/` - Workspace with feature-specific crates
- `crates/net/` - MTProto network layer (fully implemented)
- `crates/types/` - TL schema type definitions
- `crates/actor/` - Actor framework for concurrency
- `crates/storage/` - Database layer with SQLite backend
- `crates/dialog_manager/` - Central dialog management with network integration
- `references/td/` - TDLib source for reference
- `references/paper-plane/` - Telegram client on Rust developed by others developers

## Commands

```bash
# Build entire workspace
cargo build --workspace

# Build specific crate
cargo build -p net

# Run tests
cargo test --workspace

# Format check
cargo fmt -- --check

# Lint
cargo clippy --workspace -- -D warnings

# Verify single module build
tools/build-verify.sh <crate_name>

# Coverage analysis (requires cargo-llvm-cov)
cargo llvm-cov --workspace --html --output-dir coverage

# Coverage for specific crate
cargo llvm-cov -p net --html
```

## Code Style

**Rust Standards**: Follow `rust-pro` skill guidelines:
- **NO `unwrap()` in production** - use proper `Result`/`Option` handling
- Mandatory error handling with `?` operator
- Type hints for public APIs
- Doc comments on all public items

**Testing**:
- Unit tests inline with `#[cfg(test)]`
- Integration tests in `tests/` directory
- Mock external dependencies
- Property-based tests for crypto/state machines (proptest)
- Concurrency tests for actor framework

## Architecture

**Actor Model**: Components communicate via message passing
- `ActorId<T>` - Type-safe actor identifiers
- Messages implement `ActorMessage`
- Lifecycle: `Start` -> `Running` -> `Stop`

**MTProto**: Custom Telegram protocol
- AES-IGE encryption (matches TDLib)
- Multi-DC support
- Connection pooling in `crates/net/`

**TL Schema**: Type definitions generated from Telegram's schema
- Each type has dedicated crate (`user_id`, `chat_id`, etc.)
- Strong typing prevents API misuse

**Storage Layer**: SQLite-based persistence
- Versioned schema migrations
- TL types stored as BLOB for flexibility
- Separate databases for messages, users, chats, files
- See `crates/storage/README.md` for API reference

**DialogManager**: Unified dialog operations
- Dialog registration and metadata tracking
- Network integration via `NetworkClient`
- TTL-based caching layer
- Input peer construction for API calls

## Workflow

Use /orchestration skill

## Key Files

- `Cargo.toml` - Workspace configuration
- `tools/build-verify.sh` - Single crate build verification
- `tools/scorer.py` - Module complexity scoring
- `crates/dialog_manager/README.md` - DialogManager usage guide
- `crates/storage/README.md` - Database layer documentation

## Documentation

### Storage Layer (Epic: rustgram-client-a7u "Phase 1B: Database Layer")
**Status**: COMPLETE (2026-01-23)

**Implemented Databases**:
- `MessageDb` - Message storage with 5 migrations (TTL, search, scheduled)
- `UserDb` - User profile storage with 2 migrations
- `ChatDb` - Chat/group metadata with 2 migrations
- `FileDb` - File metadata storage with 1 migration

**Features**:
- Schema migrations with version tracking
- TL type storage as BLOB for flexibility
- Transaction support with `Transaction` wrapper
- Synchronous and async database interfaces
- Error handling with transient/non-transient classification

**Performance**: < 10ms single entity, < 100ms batch queries
**Test Coverage**: 121 tests (97 unit + 15 integration + 9 corruption)

**Documentation**: `crates/storage/README.md`

### DialogManager Network Integration (Epic: rustgram-client-4gl)
**Status**: COMPLETE (2026-01-19)

**Completed Components**:
- `crates/dialog_manager/src/network.rs` - Network client integration (405 lines)
- `crates/dialog_manager/src/tl_types.rs` - TL schema types (705 lines)
- `crates/dialog_manager/src/cache.rs` - TTL-based caching (599 lines)
- `crates/dialog_manager/src/lib.rs` - Main manager (1852 lines)
- `crates/dialog_manager/README.md` - Usage documentation

**Features**:
- Async/await network client wrapping callback-based NetQuery
- TL request/response types for dialog operations
- Dialog and metadata caching with automatic expiration
- Server operations: load_dialogs, create_dialog, update_title, update_photo

**Documentation Coverage**: ~100% (243 doc comment lines across all modules)

### API Compatibility Audit (Phase 6)
**Epic**: rustgram-client-1q4 "ma0.4-api-compatibility"
**Status**: COMPLETE
**Index**: `docs/api-compatibility-index.md`

**Key Findings**:
- 43% weighted API coverage across 6 critical managers
- 87% TL schema baseline coverage
- 2 high-risk items identified (network layer, database)
- Estimated 3-4 months to MVP, 6-8 months to full parity

**Documents**:
- `docs/api-compatibility-index.md` - Navigation index for all audit artifacts
- `docs/api-compatibility-report.md` - Main audit findings
- `docs/actor-pattern-mapping.md` - Architectural pattern analysis
- `docs/stub-inventory.md` - Implementation roadmap by manager
- `docs/gap-analysis.md` - Detailed gap identification and prioritization
- `docs/tl-schema-baseline.md` - TL schema coverage baseline

### Test Coverage Status
**Last Audit:** 2026-01-17 (Task: rustgram-client-mih)
**See full details:** `docs/test-coverage-audit.md`

### Current Metrics

| Category | Crates | Est. Coverage | Target | Status |
|----------|--------|---------------|--------|--------|
| Critical (P0/P1) | 13 | 38% -> 45% | 70-80% | Below Target |
| Manager | 261 | 55% -> 60% | 60%+ | Approaching |
| Type | 50 | 65% | 50%+ | Meeting Target |
| Stub | 72 | 35% | 20%+ | Exceeding Target |
| **Overall** | **396** | **52% -> 58%** | **60%+** | **Approaching** |

### Critical Module Coverage

| Module | Priority | Est. Coverage | Target | Status |
|--------|----------|---------------|--------|--------|
| net | P0 | 28% | 80% | Critical Gap |
| crypto | P0 | 56% | 80% | Needs Property Tests |
| actor | P0 | 55% | 75% | Needs Concurrency Tests |
| storage | P0 | 26% -> 85% | 75% | **Improved** |
| client_actor | P1 | 68% | 70% | Near Target |
| td_db | P1 | 67% | 70% | Near Target |
| auth_manager | P1 | 7% | 70% | Below Target |
| user_manager | P1 | 7% | 70% | Below Target |
| password_manager | P1 | 24% | 75% | Below Target |
| dialog_manager | P1 | 26% -> 60%+ | 70% | Improved (Network Integration) |
| auth | P1 | 16% | 70% | Below Target |
| message_db | P1 | 60% | 70% | Near Target |
| secure_storage | P1 | 58% | 70% | Near Target |

### Known Issues

1. **Missing Coverage Tools**: `cargo-llvm-cov` and `cargo-tarpaulin` not installed
2. **No Property-Based Tests**: Crypto operations need property-based validation
3. **No Concurrency Tests**: Actor framework lacks race condition testing
4. **Limited Integration Tests**: Only 2 crates have integration test suites

### Recommendations

1. Install coverage tools: `cargo install cargo-llvm-cov cargo-tarpaulin`
2. Run actual coverage: `cargo llvm-cov --workspace --html --output-dir coverage`
3. Add property-based tests for crypto module using `proptest`
4. Add concurrency tests for actor framework
5. Expand integration test coverage

## Important

**Before implementing**: Check TDLib reference in `references/td/` for expected behavior.

**Security**: Cryptography must match TDLib exactly - see `crates/net/` for examples.

**Testing Requirements**:
- Critical modules (net, crypto, actor) require 70-80% coverage
- Manager modules require 60%+ coverage
- Use property-based testing for crypto operations
- Use concurrency testing for actor message passing

**Database Usage**:
- See `crates/storage/README.md` for complete API reference
- TL types are stored as BLOB to avoid tight coupling
- Use migrations for schema changes
- Handle `NotFound` errors explicitly
- Retry on transient errors (DatabaseLocked, ConnectionError)
