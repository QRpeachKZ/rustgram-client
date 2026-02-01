# rustgram-connectionstate

Connection state management for Telegram MTProto client.

## Overview

This crate provides types and utilities for tracking and managing the connection state to Telegram servers. It implements the state machine from TDLib's `ConnectionStateManager` with callback support.

## Connection States

The connection state progresses through these stages:

- `Empty` - Initial state (not yet connected)
- `WaitingForNetwork` - No network connectivity
- `ConnectingToProxy` - Connecting through proxy
- `Connecting` - Connecting to Telegram servers
- `Updating` - Syncing data with servers
- `Ready` - Connection ready and operational

## Usage

### Basic State Tracking

```rust
use rustgram_connectionstate::{ConnectionStateManager, ConnectionState};

let mut manager = ConnectionStateManager::new();
assert_eq!(manager.current_state(), ConnectionState::Empty);

manager.set_state(ConnectionState::Connecting).unwrap();
assert!(manager.current_state().is_connecting());

manager.set_state(ConnectionState::Ready).unwrap();
assert!(manager.current_state().is_ready());
```

### Using Callbacks

```rust
use rustgram_connectionstate::{
    ConnectionStateManager, ConnectionState, ClosureCallback
};

// Create a callback that logs state changes
let callback = ClosureCallback::new(|state| {
    println!("State changed to: {}", state);
    true // Keep callback registered
});

let mut manager = ConnectionStateManager::new();
manager.register_callback(Box::new(callback));

// This will trigger the callback
manager.set_state(ConnectionState::Ready).unwrap();
```

### Custom Callback Implementation

```rust
use rustgram_connectionstate::{ConnectionState, StateCallback};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CounterCallback {
    counter: std::sync::Arc<AtomicUsize>,
}

impl StateCallback for CounterCallback {
    fn on_state_changed(&self, state: ConnectionState) -> bool {
        self.counter.fetch_add(1, Ordering::SeqCst);
        println!("State changed to: {} (count: {})",
            state, self.counter.load(Ordering::SeqCst));
        true
    }
}
```

## Testing

Run tests with:

```bash
cargo test -p rustgram-connectionstate
```

## TDLib Alignment

This module is based on TDLib's `ConnectionStateManager` implementation:

- Reference: `td/telegram/ConnectionStateManager.h`
- Reference: `td/telegram/ConnectionState.h`
- Reference: `td/telegram/StateManager.h`

## License

MIT OR Apache-2.0
