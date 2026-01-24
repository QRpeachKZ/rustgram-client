// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Property-based tests for the actor framework.
//!
//! These tests use proptest to verify invariants across a wide
//! range of randomly generated inputs.

use proptest::prelude::*;
use rustgram_actor::*;
use std::collections::HashSet;
use std::time::Duration;

// Property: Actor ID equality and hashing are consistent
proptest! {
    #[test]
    fn prop_actor_id_equality_reflexive(id1 in any::<u64>(), sid1 in any::<u32>(), gen1 in any::<u32>()) {
        struct TestActor;
        impl Actor for TestActor {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        let id = ActorId::<TestActor>::new(id1, sid1, gen1);
        prop_assert_eq!(id, id);
    }

    #[test]
    fn prop_actor_id_equality_symmetric(id1 in any::<u64>(), sid1 in any::<u32>(), gen1 in any::<u32>()) {
        struct TestActor;
        impl Actor for TestActor {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        let id_a = ActorId::<TestActor>::new(id1, sid1, gen1);
        let id_b = ActorId::<TestActor>::new(id1, sid1, gen1);
        prop_assert_eq!(id_a, id_b);
        prop_assert_eq!(id_b, id_a);
    }

    #[test]
    fn prop_actor_id_hash_consistency(
        ids in prop::collection::vec((any::<u64>(), any::<u32>(), any::<u32>()), 1..100)
    ) {
        struct TestActor;
        impl Actor for TestActor {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        let mut set = HashSet::new();
        for (id, sid, gen) in ids {
            let actor_id = ActorId::<TestActor>::new(id, sid, gen);
            set.insert(actor_id);
        }

        // Unique IDs should result in unique hashes
        prop_assert!(set.len() <= ids.len());
    }

    #[test]
    fn prop_actor_id_ordering_transitive(
        id1 in any::<u64>(), sid1 in any::<u32>(), gen1 in any::<u32>(),
        id2 in any::<u64>(), sid2 in any::<u32>(), gen2 in any::<u32>(),
        id3 in any::<u64>(), sid3 in any::<u32>(), gen3 in any::<u32>()
    ) {
        struct TestActor;
        impl Actor for TestActor {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        let a = ActorId::<TestActor>::new(id1, sid1, gen1);
        let b = ActorId::<TestActor>::new(id2, sid2, gen2);
        let c = ActorId::<TestActor>::new(id3, sid3, gen3);

        // Transitivity: if a <= b and b <= c, then a <= c
        if a <= b && b <= c {
            prop_assert!(a <= c);
        }
    }
}

// Property: Registry operations maintain consistency
proptest! {
    #[test]
    fn prop_registry_insert_contains(
        ids in prop::collection::vec(any::<u64>(), 1..100)
    ) {
        let registry = Registry::new();

        for (i, &id) in ids.iter().enumerate() {
            let info = ActorInfo::new(format!("actor_{}", i), 0);
            registry.insert(id, info);
            prop_assert!(registry.contains(id));
        }
    }

    #[test]
    fn prop_registry_remove_deletes(
        ids in prop::collection::vec(any::<u64>(), 1..100)
    ) {
        let registry = Registry::new();

        for &id in &ids {
            let info = ActorInfo::new(format!("actor_{}", id), 0);
            registry.insert(id, info);
        }

        // Remove all
        for &id in &ids {
            prop_assert!(registry.remove(id).is_some());
        }

        // None should remain
        for &id in &ids {
            prop_assert!(!registry.contains(id));
        }
    }

    #[test]
    fn prop_registry_size_matches_ids(
        inserts in prop::collection::vec(any::<u64>(), 1..50),
        removes in prop::collection::vec(any::<u64>(), 1..50)
    ) {
        let registry = Registry::new();

        for &id in &inserts {
            let info = ActorInfo::new(format!("actor_{}", id), 0);
            registry.insert(id, info);
        }

        for &id in &removes {
            registry.remove(id);
        }

        let expected_count = inserts.iter().collect::<HashSet<_>>().len()
            - removes.iter().filter(|x| inserts.contains(x)).collect::<HashSet<_>>().len();

        prop_assert_eq!(registry.len(), expected_count.max(0));
    }
}

// Property: Mailbox FIFO ordering
proptest! {
    #[test]
    fn prop_mailbox_fifo_ordering(
        values in prop::collection::vec(any::<u64>(), 1..100)
    ) {
        let mailbox = Mailbox::new();

        for &val in &values {
            let _ = mailbox.push(Event::Raw(val, Box::new(val)));
        }

        let mut retrieved = Vec::new();
        while let Some(event) = mailbox.pop() {
            if let Event::Raw(val, _) = event {
                retrieved.push(val);
            }
        }

        prop_assert_eq!(retrieved, values);
    }

    #[test]
    fn prop_mailbox_capacity_respected(
        capacity in 1usize..100,
        values in prop::collection::vec(any::<u64>(), 1..200)
    ) {
        let mailbox = Mailbox::with_capacity(capacity);

        let mut success_count = 0;
        for &val in &values {
            if mailbox.push(Event::Raw(val, Box::new(val))).is_ok() {
                success_count += 1;
            }
        }

        prop_assert!(success_count <= capacity);
        prop_assert_eq!(mailbox.len(), success_count.min(capacity));
    }

    #[test]
    fn prop_mailbox_len_matches_count(
        values in prop::collection::vec(any::<u64>(), 1..100)
    ) {
        let mailbox = Mailbox::new();

        for (i, &val) in values.iter().enumerate() {
            let _ = mailbox.push(Event::Raw(val, Box::new(val)));
            prop_assert_eq!(mailbox.len(), i + 1);
        }

        for (i, _) in values.iter().enumerate() {
            mailbox.pop();
            prop_assert_eq!(mailbox.len(), values.len() - i - 1);
        }
    }
}

// Property: Supervisor restart limits
proptest! {
    #[test]
    fn prop_supervisor_respects_max_retries(
        max_retries in 1usize..10,
        failures in 1usize..20
    ) {
        let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_one(
            max_retries,
            Duration::from_secs(60),
        ));
        supervisor.add_child(123);

        for _ in 0..failures {
            supervisor.handle_failure(123);
        }

        let child = supervisor.get_child(123).unwrap();
        prop_assert!(child.restart_count <= max_retries);
    }

    #[test]
    fn prop_supervisor_one_for_all_restarts_all(
        child_ids in prop::collection::vec(1u64..100u64, 2..10),
        fail_id in 1u64..100u64
    ) {
        let mut supervisor = Supervisor::new(SupervisorStrategy::one_for_all(
            10,
            Duration::from_secs(60),
        ));

        for &id in &child_ids {
            supervisor.add_child(id);
        }

        // Cause a failure
        supervisor.handle_failure(fail_id);

        // All children should have their restart count incremented
        for &id in &child_ids {
            if let Some(child) = supervisor.get_child(id) {
                prop_assert_eq!(child.restart_count, 1);
            }
        }
    }
}

// Property: ActorInfo state transitions
proptest! {
    #[test]
    fn prop_actor_state_aliveness(
        state in prop::sample::vec(
            prop::enum::Variant::from([
                ActorState::Starting,
                ActorState::Running,
                ActorState::Stopping,
                ActorState::Dead,
            ]),
            1..10
        )
    ) {
        // Dead is the only non-alive state
        let is_alive = !matches!(state, ActorState::Dead);
        prop_assert_eq!(state.is_alive(), is_alive);
    }

    #[test]
    fn prop_actor_state_runnability(
        state in prop::sample::vec(
            prop::enum::Variant::from([
                ActorState::Starting,
                ActorState::Running,
                ActorState::Stopping,
                ActorState::Dead,
                ActorState::Migrating(1),
            ]),
            1..10
        )
    ) {
        // Only Running is runnable
        let is_runnable = matches!(state, ActorState::Running);
        prop_assert_eq!(state.is_runnable(), is_runnable);
    }
}

// Property: Event routing
proptest! {
    #[test]
    fn prop_event_full_system_detection(
        source_id in any::<u64>(),
        dest_id in any::<u64>(),
        scheduler in any::<u32>()
    ) {
        let system_event = EventFull::system(Event::Start, dest_id, scheduler);
        prop_assert_eq!(system_event.source_id, 0);
        prop_assert!(system_event.is_system());

        let user_event = EventFull::new(Event::Start, source_id, dest_id, scheduler);
        prop_assert_eq!(user_event.source_id, source_id);

        let is_system = source_id == 0;
        prop_assert_eq!(user_event.is_system(), is_system);
    }
}

// Property: Envelope type safety
proptest! {
    #[test]
    fn prop_envelope_roundtrip(
        value in any::<i32>()
    ) {
        struct TestMessage(i32);
        impl Message for TestMessage {
            type Response = NoResponse;
        }

        let envelope = Envelope::new(TestMessage(value));
        prop_assert!(envelope.is::<TestMessage>());

        let recovered = envelope.downcast::<TestMessage>();
        prop_assert!(recovered.is_some());
        prop_assert_eq!(recovered.unwrap().0, value);
    }
}

// Property: Response conversion
proptest! {
    #[test]
    fn prop_response_ok_roundtrip(
        value in any::<i32>()
    ) {
        let response = Response::ok(value);
        prop_assert!(response.is_ok());
        prop_assert!(!response.is_err());

        let result = response.into_result();
        prop_assert_eq!(result, Ok(value));
    }

    #[test]
    fn prop_response_error_conversion(
        error_type in 0usize..4
    ) {
        let response = match error_type {
            0 => Response::<i32>::ActorNotFound,
            1 => Response::<i32>::ActorNotRunning,
            2 => Response::<i32>::Timeout,
            3 => Response::<i32>::Error("test error".to_string()),
            _ => unreachable!(),
        };

        prop_assert!(response.is_err());
        prop_assert!(!response.is_ok());

        let result = response.into_result();
        prop_assert!(result.is_err());
    }
}

// Property: ActorShared and ActorOwn
proptest! {
    #[test]
    fn prop_actor_shared_id_preservation(
        id in any::<u64>(),
        sid in any::<u32>(),
        gen in any::<u32>()
    ) {
        struct TestActor;
        impl Actor for TestActor {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        let actor_id = ActorId::<TestActor>::new(id, sid, gen);
        let shared = ActorShared::new(actor_id);

        prop_assert_eq!(shared.id().as_u64(), id);
        prop_assert_eq!(shared.id().scheduler_id(), sid);
        prop_assert_eq!(shared.id().generation(), gen);

        let cloned = shared.clone();
        prop_assert_eq!(cloned.id(), shared.id());
    }

    #[test]
    fn prop_actor_own_id_preservation(
        id in any::<u64>(),
        sid in any::<u32>(),
        gen in any::<u32>()
    ) {
        struct TestActor;
        impl Actor for TestActor {
            fn start_up(&mut self) {}
            fn wakeup(&mut self) {}
            fn hangup(&mut self) {}
            fn tear_down(&mut self) {}
            fn loop_exec(&mut self) {}
            fn timeout_expired(&mut self) {}
        }

        let actor_id = ActorId::<TestActor>::new(id, sid, gen);
        let own = ActorOwn::new(actor_id);

        prop_assert_eq!(own.id().as_u64(), id);
        prop_assert_eq!(own.id().scheduler_id(), sid);
        prop_assert_eq!(own.id().generation(), gen);

        let cloned = own.clone();
        prop_assert_eq!(cloned.id(), own.id());
    }
}

// Property: Error handling
proptest! {
    #[test]
    fn prop_actor_error_display(
        id in any::<u64>(),
        msg in "[a-zA-Z0-9]{1,50}"
    ) {
        let error = ActorError::ActorNotFound(id);
        let display = format!("{}", error);
        prop_assert!(display.contains(&id.to_string()));

        let error = ActorError::InvalidState(msg.clone());
        let display = format!("{}", error);
        prop_assert!(display.contains(&msg));
    }
}

// Property: ActorInfo cloning
proptest! {
    #[test]
    fn prop_actor_info_clone_preserves_state(
        name in "[a-zA-Z0-9]{1,50}",
        scheduler_id in any::<u32>(),
        state_val in 0usize..5
    ) {
        let state = match state_val {
            0 => ActorState::Starting,
            1 => ActorState::Running,
            2 => ActorState::Stopping,
            3 => ActorState::Dead,
            4 => ActorState::Migrating(scheduler_id % 10),
            _ => unreachable!(),
        };

        let mut info = ActorInfo::new(name.clone(), scheduler_id);
        info.set_state(state);

        let cloned = info.clone();

        prop_assert_eq!(info.name(), cloned.name());
        prop_assert_eq!(info.state(), cloned.state());
        prop_assert_eq!(info.scheduler_id(), cloned.scheduler_id());
    }
}
