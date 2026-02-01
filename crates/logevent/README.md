# rustgram-logevent

TDLib-compatible log event module for Telegram client.

## Overview

This module implements TDLib's event logging system for persisting operations that need to survive application restarts. It handles serialization and deserialization using TL (Type Language) format.

## Structure

```
crates/logevent/
├── Cargo.toml                    # Package configuration
├── src/
│   ├── lib.rs                    # Public API and documentation
│   ├── error.rs                  # Error types (LogEventError)
│   ├── types.rs                  # Core types (HandlerType, LogEventIdWithGeneration)
│   ├── flags.rs                  # Flag handling (FlagsStorer, FlagsParser)
│   ├── parser.rs                 # TL parsing (TlParser, LogEventParser)
│   ├── storer.rs                 # TL storing (TlStorer, LogEventStorer...)
│   ├── secret.rs                 # Secret chat events
│   └── helper.rs                 # Helper functions
└── tests/
    ├── test_types.rs             # Core type tests
    ├── test_serialization.rs     # Serialization tests
    └── test_secret_events.rs     # Secret event tests
```

## Key Types

### Core Traits
- `LogEvent`: Base trait for all log events
- `TlParser`: TL parsing trait
- `TlStorer`: TL storing trait
- `SecretChatEvent`: Base trait for secret chat events

### Handler Types
All 60+ handler types from TDLib are supported:
- `SecretChats`, `Users`, `Chats`, `Channels`
- `SendMessage`, `DeleteMessage`, `ReadHistoryOnServer`
- `DeleteStoryOnServer`, `SendStory`, etc.

### Secret Chat Events
- `InboundSecretMessage`: Received secret messages
- `OutboundSecretMessage`: Sent secret messages
- `CloseSecretChat`: Secret chat closure
- `CreateSecretChat`: Secret chat creation

### Time Helpers
- `store_time`: Store timestamps with server time (TDLib-compatible)
- `parse_time`: Parse timestamps with server time adjustment

## Usage Example

```rust
use rustgram_logevent::{
    LogEvent, CreateSecretChat, LogEventStorerVec, LogEventParser,
};

// Create a secret chat event
let event = CreateSecretChat::new(12345, 67890, 11111);

// Serialize
let mut storer = LogEventStorerVec::new();
event.store(&mut storer);
let data = storer.into_inner();

// Deserialize
let mut parser = LogEventParser::new(&data);
let parsed = CreateSecretChat::parse(&mut parser)?;
```

## Status

- [x] Core types implemented (100%)
- [x] TL parser/storer implemented (100%)
- [x] Flag handling implemented (100%)
- [x] Secret chat events implemented (80% - secret_api types pending)
- [x] Time helpers implemented (100%)
- [x] Unit tests written (130+ tests)
- [ ] Binlog integration (TODO - depends on binlog module)
- [ ] Secret API types (TODO - separate module)

## Dependencies

- `bytes`: Byte buffer utilities
- `byteorder`: Endianness handling
- `thiserror`: Error derive macros
- `tracing`: Logging
- `tokio`: Async runtime (workspace)

## Testing

Run tests with:
```bash
cargo test -p rustgram-logevent
```

## TDLib Reference

This implementation is based on TDLib source:
- `references/td/td/telegram/logevent/LogEvent.h`
- `references/td/td/telegram/logevent/LogEventHelper.h`
- `references/td/td/telegram/logevent/SecretChatEvent.h`
