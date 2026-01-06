# Telegram Client Rust Rewrite

## Project Overview
Native Rust implementation of Telegram client based on TDLib/MTProto.

### Reference Implementation
- **TDLib (Telegram Database Library)**: `references/td/td/telegram/`
- **Official Repository**: https://github.com/tdlib/td
- **MTProto Protocol**: https://core.telegram.org/mtproto
- **TL Language**: https://core.telegram.org/tl

### One-Shot Agentic Coding Approach
This project uses the One-Shot Agentic methodology:
- Each module is completed in ONE prompt attempt
- Comprehensive context provided upfront
- REPLACEMENT-LEVEL CODEBASE GRAPH NAVIGATION via MCP code-grapher
- INTERACTIVE CODING ASSISTANT via Claude Code
- DELIBERATIVE EXECUTION through agentic loops
- COMPREHEND-CODE-GEN LOOP for analysis and generation

## Module Development Workflow

### 1. Select Module
Use `tools/scorer.py` to identify modules by complexity:
```bash
python3 tools/scorer.py
```
Start with LOW complexity modules first.

### 2. Comprehend Phase (One-Shot)
Gather ALL context before coding:
- Read official TDLib implementation in `references/td/td/telegram/<module>/`
- Review MTProto specification for relevant protocol messages
- Use MCP code-grapher to understand dependencies and call graphs
- Study type definitions and API contracts

### 3. Code-Gen Phase (One-Shot)
Implement complete module in single attempt:
- Create Rust types matching TDLib structures
- Implement protocol handling logic
- Add comprehensive error handling
- Write unit tests covering key paths
- Document all public APIs

### 4. Verify
```bash
./tools/build-verify.sh <module>
```
This runs:
- Cargo fmt check
- Clippy lints
- Build verification
- Unit tests
- Doc tests
- Compliance tests (if available)

### 5. Commit
If build passes + tests pass:
```bash
git add crates/<module>/
git commit -m "feat: implement <module> module"
```

### 6. Handle Failures
If verification fails > 5 attempts:
```bash
echo "<module>: <reason>" >> difficult_modules.txt
```
Skip and return later with different approach.

## Module Template

Each module should follow this structure:

```
crates/<module>/
  Cargo.toml
  src/
    lib.rs          # Public API, documentation
    types.rs        # Type definitions
    error.rs        # Error types
    private.rs      # Internal implementation
  compliance_test.sh   # Optional: vs TDLib
```

## Coding Standards

### Rust Standards
- No `.unwrap()` in production code
- Mandatory `Result`/`Option` error handling
- `#[deny(clippy::all)]` enabled
- Documentation on all public APIs

### TDLib Alignment
- Match TDLib's API structure where applicable
- Preserve type semantics from official implementation
- Document deviations with rationale

### Testing Requirements
- Unit tests for all public functions
- Property-based tests for state machines
- Protocol compliance tests for network interactions

## Tools

### Available Tools
| Tool | Purpose |
|------|---------|
| `tools/scorer.py` | Module complexity analysis |
| `tools/build-verify.sh` | Build and verification |
| MCP code-grapher | Codebase navigation |
| `references/td/` | Official TDLib source |

### MCP Tools (pre-approved)
- `mcp__code-grapher__*` - All code-grapher operations
- `Skill(smart-search)` - Web search for protocol specs

## Module Dependencies

Typical dependency order (low to high complexity):
1. **types** - Basic type definitions
2. **net** - Network types (DC ID, DC options)
3. **config** - Configuration management (COMPLETED)
4. **auth** - Authentication flow
5. **storage** - Local data persistence
6. **sync** - Data synchronization
7. **encryption** - MTProto encryption
8. **network** - Network layer
9. **messages** - Message handling
10. **calls** - Voice/video calls
11. **ui** - UI integration

## Module Tracking

**📊 Implementation Status:** `.claude/workspace/modules_tracker.md`

This tracker maintains:
- ✅ Implemented modules (11 completed)
- ❌ Queue of ~189 remaining modules, ordered by complexity
- 🔄 In-progress assignments
- LOC counts, test coverage, and dependencies

**Quick stats:**
- Total TDLib LOC: ~150,000+
- Implemented: ~12,410 LOC (8.3%)
- Total tests: 775
- Average: 70 tests per module

**Usage:**
1. Check tracker for next module to implement
2. Update status when starting/finishing
3. Mark completed modules after commit

## Implemented Modules

See `.claude/workspace/modules_tracker.md` for complete list.

### config (v0.1.0) ✅
Configuration management for Telegram MTProto client.

**Components:**
- `SimpleConfig` - Basic connection configuration (DC options, default DC)
- `AppConfig` - Application-level configuration with versioning and caching (v110)
- `DhConfig` - Diffie-Hellman parameters for key exchange
- `ConfigManager` - Centralized management with caching and updates
- TL constructors for MTProto protocol

**Testing:** 48 unit tests + 5 doctests

**Documentation:** `config-module.md`

### Other Completed Modules 🟢
- **net** (338 tests) - Network layer, MTProto, transports
- **auth** (41 tests) - Authentication flow
- **storage** (24 tests) - Local data persistence
- **logevent** (65 tests) - Event logging for binlog
- **venue** (48 tests) - Location and venue objects
- **birthdate** (33 tests) - Birthdate types
- **boost** (30 tests) - Boost management
- **connectionstate** (36 tests) - Connection state types
- **theme** (11 tests) - Theme manager and settings
- **types** (46 tests) - Base type definitions

**Main rule:** 
- 1 session – 300 summary LOC

## Example Workflow

```bash
# 1. Score modules
python3 tools/scorer.py

# 2. Select module (e.g., "auth")
# 3. Read TDLib auth implementation
# 4. Use code-grapher to analyze dependencies
# 5. Implement in crates/auth/

# 6. Verify
./tools/build-verify.sh auth

# 7. If successful
git add crates/auth/
git commit -m "feat: implement auth module"
```

## Important Notes

- **One-Shot**: Don't iterate. Get context right, then implement once.
- **Context First**: Use MCP tools to understand before coding.
- **Test-Driven**: Write tests alongside implementation.
- **Fail Fast**: Log difficult modules and move on.
- **Reference**: Always compare with TDLib implementation.

## References
- TDLib Source: `references/td/`
- MTProto 2.0: https://core.telegram.org/mtproto
- TL Language: https://core.telegram.org/tl
- TDLib API: https://core.telegram.org/tdlib
