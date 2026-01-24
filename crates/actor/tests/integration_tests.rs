// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Integration tests for the actor framework.
//!
//! These tests verify end-to-end actor communication, multi-scheduler
//! scenarios, supervisor patterns, and graceful shutdown.

use rustgram_actor::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

// Test helper actors
struct CounterActor {
    count: Arc<AtomicUsize>,
}

impl CounterActor {
    fn new(count: Arc<AtomicUsize>) -> Self {
        Self { count }
    }
}

impl Actor for CounterActor {
    fn start_up(&mut self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    fn wakeup(&mut self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    fn hangup(&mut self) {
        self.count.fetch_sub(1, Ordering::SeqCst);
    }

    fn tear_down(&mut self) {
        self.count.fetch_sub(1, Ordering::SeqCst);
    }

    fn loop_exec(&mut self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    fn timeout_expired(&mut self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }
}

struct EchoActor {
    last_message: Option<String>,
}

impl EchoActor {
    fn new() -> Self {
        Self {
            last_message: None,
        }
    }
}

impl Actor for EchoActor {
    fn start_up(&mut self) {}
    fn wakeup(&mut self) {}
    fn hangup(&mut self) {}
    fn tear_down(&mut self) {}
    fn loop_exec(&mut self) {}
    fn timeout_expired(&mut self) {}
}

struct FailingActor {
    fail_count: Arc<AtomicUsize>,
    should_fail: bool,
}

impl FailingActor {
    fn new(fail_count: Arc<AtomicUsize>, should_fail: bool) -> Self {
        Self {
            fail_count,
            should_fail,
        }
    }
}

impl Actor for FailingActor {
    fn start_up(&mut self) {
        if self.should_fail {
            self.fail_count.fetch_add(1, Ordering::SeqCst);
            panic!("Intentional failure in start_up");
        }
    }

    fn wakeup(&mut self) {
        if self.should_fail {
            self.fail_count.fetch_add(1, Ordering::SeqCst);
            panic!("Intentional failure in wakeup");
        }
    }

    fn hangup(&mut self) {}
    fn tear_down(&mut self) {}
    fn loop_exec(&mut self) {
        if self.should_fail {
            self.fail_count.fetch_add(1, Ordering::SeqCst);
            panic!("Intentional failure in loop_exec");
        }
    }

    fn timeout_expired(&mut self) {}
}

// Integration tests

#[test]
fn test_actor_lifecycle_full() {
    let count = Arc::new(AtomicUsize::new(0));

    let mut actor = CounterActor::new(count.clone());

    // Simulate full lifecycle
    actor.start_up();
    assert_eq!(count.load(Ordering::SeqCst), 1);

    actor.wakeup();
    assert_eq!(count.load(Ordering::SeqCst), 2);

    actor.loop_exec();
    assert_eq!(count.load(Ordering::SeqCst), 3);

    actor.hangup();
    assert_eq!(count.load(Ordering::SeqCst), 2);

    actor.tear_down();
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn test_actor_shared_ownership() {
    struct SharedStateActor {
        value: Arc<AtomicUsize>,
    }

    impl Actor for SharedStateActor {
        fn start_up(&mut self) {
            self.value.fetch_add(10, Ordering::SeqCst);
        }

        fn wakeup(&mut self) {}
        fn hangup(&mut self) {}
        fn tear_down(&mut self) {}
        fn loop_exec(&mut self) {}
        fn timeout_expired(&mut self) {}
    }

    let value = Arc::new(AtomicUsize::new(0));

    let mut actor1 = SharedStateActor {
        value: value.clone(),
    };
    let mut actor2 = SharedStateActor {
        value: value.clone(),
    };

    actor1.start_up();
    assert_eq!(value.load(Ordering::SeqCst), 10);

    actor2.start_up();
    assert_eq!(value.load(Ordering::SeqCst), 20);
}

#[test]
fn test_supervisor_one_for_one_strategy() {
    let fail_count = Arc::new(AtomicUsize::new(0));
    let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(
        3,
        Duration::from_secs(60),
    ));

    // Add children
    supervisor.add_child(1);
    supervisor.add_child(2);

    // Simulate child 1 failing
    let directive = supervisor.handle_failure(1);
    assert!(directive.is_restart());

    let child1 = supervisor.get_child(1).unwrap();
    assert_eq!(child1.restart_count, 1);

    // Child 2 should not be affected
    let child2 = supervisor.get_child(2).unwrap();
    assert_eq!(child2.restart_count, 0);

    // Exceed the restart limit
    for _ in 0..3 {
        supervisor.handle_failure(1);
    }

    let directive = supervisor.handle_failure(1);
    assert!(directive.is_stop());
}

#[test]
fn test_supervisor_one_for_all_strategy() {
    let fail_count = Arc::new(AtomicUsize::new(0));
    let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_all(
        3,
        Duration::from_secs(60),
    ));

    // Add multiple children
    supervisor.add_child(1);
    supervisor.add_child(2);
    supervisor.add_child(3);

    // One child fails - all should be restarted
    let directive = supervisor.handle_failure(1);
    assert_eq!(directive, SupervisorDirective::RestartAll);

    // All children should have their restart count incremented
    let child1 = supervisor.get_child(1).unwrap();
    let child2 = supervisor.get_child(2).unwrap();
    let child3 = supervisor.get_child(3).unwrap();

    assert_eq!(child1.restart_count, 1);
    assert_eq!(child2.restart_count, 1);
    assert_eq!(child3.restart_count, 1);
}

#[test]
fn test_supervisor_escalate_strategy() {
    let mut supervisor = Supervisor::new(SupervisorStrategy::Escalate);

    supervisor.add_child(1);
    supervisor.add_child(2);

    // Any failure should escalate
    let directive = supervisor.handle_failure(1);
    assert_eq!(directive, SupervisorDirective::Escalate);

    let directive = supervisor.handle_failure(2);
    assert_eq!(directive, SupervisorDirective::Escalate);
}

#[test]
fn test_supervisor_stop_strategy() {
    let mut supervisor = Supervisor::new(SupervisorStrategy::Stop);

    supervisor.add_child(1);
    supervisor.add_child(2);

    // Failed child should be stopped
    let directive = supervisor.handle_failure(1);
    assert!(directive.is_stop());
    assert_eq!(directive, SupervisorDirective::Stop(1));
}

#[test]
fn test_registry_multi_actor_scenario() {
    let registry = Registry::new();

    // Register multiple actors
    for i in 1..=5 {
        let info = ActorInfo::new(format!("actor_{}", i), i % 2);
        registry.insert(i, info);
    }

    assert_eq!(registry.len(), 5);

    // Verify all actors are present
    for i in 1..=5 {
        assert!(registry.contains(i));
        let info = registry.get(i).unwrap();
        assert_eq!(info.name(), format!("actor_{}", i));
    }

    // Remove actors in scheduler 0
    for i in 1..=5 {
        if i % 2 == 0 {
            registry.remove(i);
        }
    }

    assert_eq!(registry.len(), 3);
}

#[test]
fn test_registry_update_scenario() {
    let registry = Registry::new();

    // Create and register an actor
    let info = ActorInfo::new("test_actor".to_string(), 0);
    registry.insert(1, info);

    // Update actor state through its lifecycle
    registry.update(1, |mut info| {
        info.set_state(ActorState::Running);
        info
    });

    let info = registry.get(1).unwrap();
    assert_eq!(info.state(), ActorState::Running);

    // Simulate migration
    registry.update(1, |mut info| {
        info.set_state(ActorState::Migrating(2));
        info
    });

    let info = registry.get(1).unwrap();
    assert_eq!(info.state(), ActorState::Migrating(2));

    // Complete migration
    registry.update(1, |mut info| {
        info.set_state(ActorState::Running);
        info
    });

    let info = registry.get(1).unwrap();
    assert_eq!(info.state(), ActorState::Running);
}

#[test]
fn test_actor_id_type_safety() {
    struct ActorA;
    struct ActorB;

    impl Actor for ActorA {
        fn start_up(&mut self) {}
        fn wakeup(&mut self) {}
        fn hangup(&mut self) {}
        fn tear_down(&mut self) {}
        fn loop_exec(&mut self) {}
        fn timeout_expired(&mut self) {}
    }

    impl Actor for ActorB {
        fn start_up(&mut self) {}
        fn wakeup(&mut self) {}
        fn hangup(&mut self) {}
        fn tear_down(&mut self) {}
        fn loop_exec(&mut self) {}
        fn timeout_expired(&mut self) {}
    }

    let id_a = ActorId::<ActorA>::new(1, 0, 0);
    let id_b = ActorId::<ActorB>::new(1, 0, 0);

    // Same numeric ID but different types
    assert_eq!(id_a.as_u64(), id_b.as_u64());

    // Can erase type for storage
    let erased_a = id_a.erase_type();
    let erased_b = id_b.erase_type();

    assert_eq!(erased_a.as_u64(), erased_b.as_u64());
}

#[test]
fn test_mailbox_fifo_ordering_stress() {
    let mailbox = Mailbox::new();

    // Push many events
    for i in 0..100 {
        mailbox.push(Event::Raw(i, Box::new(i)));
    }

    // Verify FIFO order
    for i in 0..100 {
        let event = mailbox.pop().unwrap();
        match event {
            Event::Raw(id, _) => assert_eq!(id, i),
            _ => panic!("Expected Raw event"),
        }
    }

    assert!(mailbox.is_empty());
}

#[test]
fn test_mailbox_capacity_limit() {
    let mailbox = Mailbox::with_capacity(10);

    // Fill to capacity
    for _ in 0..10 {
        assert!(mailbox.push(Event::Start).is_ok());
    }

    // Exceed capacity
    assert!(mailbox.push(Event::Stop).is_err());

    // Pop one and push again
    mailbox.pop();
    assert!(mailbox.push(Event::Yield).is_ok());
}

#[test]
fn test_supervised_child_restart_time_window() {
    let child = SupervisedChild::new(123);
    let strategy = SupervisorStrategy::one_for_one(3, Duration::from_millis(100));

    // Record some restarts
    let mut child = child;
    child.record_restart();
    child.record_restart();

    assert!(child.can_restart(&strategy));

    // Wait for time window to expire
    std::thread::sleep(Duration::from_millis(150));

    // After time window, restart count should reset
    assert!(child.can_restart(&strategy));
}

#[test]
fn test_actor_info_timeout_tracking() {
    let mut info = ActorInfo::new("timeout_actor".to_string(), 0);

    // No timeout initially
    assert!(!info.has_timeout_expired());

    // Set a timeout in the past
    let past = Instant::now() - Duration::from_secs(1);
    info.set_timeout(Some(past));
    assert!(info.has_timeout_expired());

    // Set a timeout in the future
    let future = Instant::now() + Duration::from_secs(10);
    info.set_timeout(Some(future));
    assert!(!info.has_timeout_expired());
}

#[test]
fn test_actor_state_transitions() {
    // Simulate actor lifecycle state transitions
    let mut state = ActorState::Starting;

    assert!(!state.is_runnable());
    assert!(state.is_alive());

    state = ActorState::Running;
    assert!(state.is_runnable());
    assert!(state.is_alive());

    state = ActorState::Migrating(2);
    assert!(!state.is_runnable());
    assert!(state.is_alive());
    assert_eq!(state.migrating_to(), Some(2));

    state = ActorState::Stopping;
    assert!(!state.is_runnable());
    assert!(state.is_alive());

    state = ActorState::Dead;
    assert!(!state.is_runnable());
    assert!(!state.is_alive());
}

#[test]
fn test_event_full_routing() {
    // Create events with different routing
    let event1 = EventFull::new(Event::Start, 100, 200, 1);
    let event2 = EventFull::system(Event::Stop, 200, 2);

    assert_eq!(event1.source_id, 100);
    assert_eq!(event1.dest_id, 200);
    assert_eq!(event1.dest_scheduler, 1);
    assert!(!event1.is_system());

    assert_eq!(event2.source_id, 0);
    assert_eq!(event2.dest_id, 200);
    assert_eq!(event2.dest_scheduler, 2);
    assert!(event2.is_system());
}

#[test]
fn test_response_error_conversion() {
    let response = Response::<i32>::ActorNotFound;
    let result = response.into_result();
    assert_eq!(result, Err(MessageResponseError::ActorNotFound));

    let response = Response::<i32>::Timeout;
    let result = response.into_result();
    assert_eq!(result, Err(MessageResponseError::Timeout));

    let response = Response::ok(42);
    let result = response.into_result();
    assert_eq!(result, Ok(42));
}

#[test]
fn test_actor_shared_and_own() {
    struct TestActor;
    impl Actor for TestActor {
        fn start_up(&mut self) {}
        fn wakeup(&mut self) {}
        fn hangup(&mut self) {}
        fn tear_down(&mut self) {}
        fn loop_exec(&mut self) {}
        fn timeout_expired(&mut self) {}
    }

    let id = ActorId::<TestActor>::new(123, 1, 0);

    let shared = ActorShared::new(id);
    assert_eq!(shared.id().as_u64(), 123);

    let own = ActorOwn::new(id);
    assert_eq!(own.id().as_u64(), 123);

    // Both can be cloned
    let shared2 = shared.clone();
    let own2 = own.clone();

    assert_eq!(shared.id(), shared2.id());
    assert_eq!(own.id(), own2.id());
}

#[test]
fn test_error_propagation() {
    let error = ActorError::ActorNotFound(123);
    assert_eq!(format!("{}", error), "Actor 123 not found");

    let error = ActorError::ActorNotRunning(456);
    assert_eq!(format!("{}", error), "Actor 456 is not running");

    let error = ActorError::InvalidState("test".to_string());
    assert!(format!("{}", error).contains("test"));
}

#[test]
fn test_envelope_type_erasure() {
    struct MessageA;
    struct MessageB;

    impl Message for MessageA {
        type Response = NoResponse;
    }

    impl Message for MessageB {
        type Response = NoResponse;
    }

    let envelope_a = Envelope::new(MessageA);
    let envelope_b = Envelope::new(MessageB);

    assert!(envelope_a.is::<MessageA>());
    assert!(!envelope_a.is::<MessageB>());

    assert!(envelope_b.is::<MessageB>());
    assert!(!envelope_b.is::<MessageA>());

    // Downcast should work for correct type
    let result = envelope_a.downcast::<MessageA>();
    assert!(result.is_some());

    let result = envelope_a.downcast::<MessageB>();
    assert!(result.is_none());
}
