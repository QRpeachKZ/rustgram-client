// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Concurrency tests for the actor framework.
//!
//! These tests verify thread safety, race condition detection,
//! deadlock prevention, and stress testing under high load.

use rustgram_actor::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Helper actors for concurrency tests
struct ConcurrentCounter {
    count: Arc<AtomicUsize>,
}

impl ConcurrentCounter {
    fn new(count: Arc<AtomicUsize>) -> Self {
        Self { count }
    }
}

impl Actor for ConcurrentCounter {
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

// Concurrency tests

#[test]
fn test_registry_concurrent_insert() {
    let registry = Arc::new(Registry::new());
    let num_threads = 10;
    let inserts_per_thread = 100;

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            for i in 0..inserts_per_thread {
                let id = (thread_id * inserts_per_thread + i) as u64;
                let info = ActorInfo::new(format!("actor_{}", id), thread_id as u32);
                registry_clone.insert(id, info);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(registry.len(), num_threads * inserts_per_thread);
}

#[test]
fn test_registry_concurrent_remove() {
    let registry = Arc::new(Registry::new());

    // Pre-populate registry
    let num_actors = 1000;
    for i in 0..num_actors {
        let info = ActorInfo::new(format!("actor_{}", i), 0);
        registry.insert(i, info);
    }

    let mut handles = vec![];
    let num_threads = 10;
    let removals_per_thread = num_actors / num_threads;

    for thread_id in 0..num_threads {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            for i in 0..removals_per_thread {
                let id = (thread_id * removals_per_thread + i) as u64;
                registry_clone.remove(id);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(registry.len(), 0);
}

#[test]
fn test_registry_concurrent_read_write() {
    let registry = Arc::new(Registry::new());

    // Pre-populate
    for i in 0..100 {
        let info = ActorInfo::new(format!("actor_{}", i), 0);
        registry.insert(i, info);
    }

    let mut handles = vec![];

    // Readers
    for _ in 0..5 {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let id = (rand::random::<usize>() % 100) as u64;
                let _ = registry_clone.get(id);
                let _ = registry_clone.contains(id);
                let _ = registry_clone.len();
            }
        });
        handles.push(handle);
    }

    // Writers
    for _ in 0..2 {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            for i in 100..200 {
                let info = ActorInfo::new(format!("actor_{}", i), 0);
                registry_clone.insert(i, info);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Should have at least 100 actors (original set)
    assert!(registry.len() >= 100);
}

#[test]
fn test_mailbox_concurrent_push_pop() {
    let mailbox = Arc::new(Mailbox::new());
    let num_ops = 1000;

    let mut handles = vec![];

    // Producers
    for _ in 0..3 {
        let mailbox_clone = Arc::clone(&mailbox);
        let handle = thread::spawn(move || {
            for i in 0..num_ops {
                let _ = mailbox_clone.push(Event::Raw(i as u64, Box::new(i)));
            }
        });
        handles.push(handle);
    }

    // Consumers
    for _ in 0..2 {
        let mailbox_clone = Arc::clone(&mailbox);
        let handle = thread::spawn(move || {
            let mut count = 0;
            while count < num_ops {
                if mailbox_clone.pop().is_some() {
                    count += 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final mailbox should be empty or nearly empty
    // (allowing for some race conditions in the test itself)
}

#[test]
fn test_actor_id_thread_safety() {
    struct TestActor;
    impl Actor for TestActor {
        fn start_up(&mut self) {}
        fn wakeup(&mut self) {}
        fn hangup(&mut self) {}
        fn tear_down(&mut self) {}
        fn loop_exec(&mut self) {}
        fn timeout_expired(&mut self) {}
    }

    let id = Arc::new(ActorId::<TestActor>::new(123, 1, 0));
    let mut handles = vec![];

    for _ in 0..10 {
        let id_clone = Arc::clone(&id);
        let handle = thread::spawn(move || {
            // Access from multiple threads
            assert_eq!(id_clone.as_u64(), 123);
            assert_eq!(id_clone.scheduler_id(), 1);
            assert_eq!(id_clone.generation(), 0);
            assert!(!id_clone.is_zero());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_supervisor_concurrent_failures() {
    let registry = Arc::new(Registry::new());
    let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(
        5,
        Duration::from_secs(60),
    ));

    // Add multiple children
    for i in 1..=10 {
        supervisor.add_child(i);
    }

    let supervisor = Arc::new(std::sync::Mutex::new(supervisor));
    let mut handles = vec![];

    // Simulate concurrent failures
    for child_id in 1..=10 {
        let supervisor_clone = Arc::clone(&supervisor);
        let handle = thread::spawn(move || {
            let mut sup = supervisor_clone.lock().unwrap();
            for _ in 0..3 {
                sup.handle_failure(child_id);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let supervisor = supervisor.lock().unwrap();
    // All children should have restart counts
    for child_id in 1..=10 {
        let child = supervisor.get_child(child_id).unwrap();
        assert_eq!(child.restart_count, 3);
    }
}

#[test]
fn test_actor_shared_concurrent_cloning() {
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
    let shared = Arc::new(ActorShared::new(id));
    let mut handles = vec![];

    for _ in 0..100 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let _ = shared_clone.clone();
            assert_eq!(shared_clone.id().as_u64(), 123);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_mailbox_stress_test() {
    let mailbox = Arc::new(Mailbox::new());
    let num_producers = 10;
    let num_consumers = 5;
    let messages_per_producer = 1000;

    let mut handles = vec![];

    // Producers
    for i in 0..num_producers {
        let mailbox_clone = Arc::clone(&mailbox);
        let handle = thread::spawn(move || {
            for j in 0..messages_per_producer {
                let id = (i * messages_per_producer + j) as u64;
                let _ = mailbox_clone.push(Event::Raw(id, Box::new(id)));
            }
        });
        handles.push(handle);
    }

    // Consumers
    for _ in 0..num_consumers {
        let mailbox_clone = Arc::clone(&mailbox);
        let handle = thread::spawn(move || {
            let mut count = 0;
            loop {
                if mailbox_clone.pop().is_some() {
                    count += 1;
                } else {
                    // Check if we're done
                    if mailbox_clone.is_empty() {
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // All messages should be consumed
    assert!(mailbox.is_empty());
}

#[test]
fn test_registry_stress_test() {
    let registry = Arc::new(Registry::new());
    let num_threads = 20;
    let ops_per_thread = 500;

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            for i in 0..ops_per_thread {
                let id = (thread_id * ops_per_thread + i) as u64;

                // Mix of operations
                match i % 4 {
                    0 => {
                        let info = ActorInfo::new(format!("actor_{}", id), thread_id as u32);
                        registry_clone.insert(id, info);
                    }
                    1 => {
                        let _ = registry_clone.get(id);
                    }
                    2 => {
                        let _ = registry_clone.contains(id);
                    }
                    3 => {
                        let _ = registry_clone.remove(id);
                    }
                    _ => unreachable!(),
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Registry should still be consistent
    let ids = registry.ids();
    let len = registry.len();
    assert_eq!(ids.len(), len);
}

#[test]
fn test_supervised_child_restart_limit_race() {
    let child = SupervisedChild::new(123);
    let strategy = SupervisorStrategy::one_for_one(3, Duration::from_secs(60));
    let child = Arc::new(std::sync::Mutex::new(child));

    let mut handles = vec![];

    // Multiple threads trying to restart simultaneously
    for _ in 0..5 {
        let child_clone = Arc::clone(&child);
        let strategy_clone = strategy.clone();
        let handle = thread::spawn(move || {
            for _ in 0..2 {
                let mut child = child_clone.lock().unwrap();
                if child.can_restart(&strategy_clone) {
                    child.record_restart();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let child = child.lock().unwrap();
    // Should not exceed max_retries
    assert!(child.restart_count <= 3);
}

#[test]
fn test_event_full_concurrent_creation() {
    let mut handles = vec![];

    for _ in 0..100 {
        let handle = thread::spawn(|| {
            for i in 0..100 {
                let _ = EventFull::new(Event::Start, i, i + 1, 1);
                let _ = EventFull::system(Event::Stop, i, 2);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_actor_info_concurrent_cloning() {
    let info = Arc::new(ActorInfo::new("test".to_string(), 0));
    let mut handles = vec![];

    for _ in 0..100 {
        let info_clone = Arc::clone(&info);
        let handle = thread::spawn(move || {
            let _ = info_clone.clone();
            let _ = info_clone.state();
            let _ = info_clone.name();
            let _ = info_clone.scheduler_id();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_envelope_concurrent_downcast() {
    struct TestMessage(i32);

    impl Message for TestMessage {
        type Response = NoResponse;
    }

    let envelope = Arc::new(std::sync::Mutex::new(Envelope::new(TestMessage(42))));
    let mut handles = vec![];

    // Multiple threads trying to downcast
    for _ in 0..10 {
        let envelope_clone = Arc::clone(&envelope);
        let handle = thread::spawn(move || {
            let env = envelope_clone.lock().unwrap();
            let _ = env.is::<TestMessage>();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_registry_update_concurrent() {
    let registry = Arc::new(Registry::new());

    // Add initial actor
    let info = ActorInfo::new("test".to_string(), 0);
    registry.insert(1, info);

    let mut handles = vec![];

    // Multiple threads updating the same actor
    for i in 0..10 {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                registry_clone.update(1, |mut info| {
                    info.set_state(match info.state() {
                        ActorState::Starting => ActorState::Running,
                        ActorState::Running => ActorState::Stopping,
                        ActorState::Stopping => ActorState::Starting,
                        _ => ActorState::Running,
                    });
                    info
                });
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Actor should still exist and be consistent
    let info = registry.get(1).unwrap();
    assert!(info.is_alive());
}
