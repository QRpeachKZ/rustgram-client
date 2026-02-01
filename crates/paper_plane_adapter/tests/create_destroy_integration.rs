// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Integration tests for RustgramClient::create() and ::destroy() methods.

#![warn(missing_docs)]
#![warn(clippy::all)]

use paper_plane_adapter::{ClientConfig, ClientId, RustgramClient};

/// Integration test for create/destroy lifecycle.
///
/// This test verifies:
/// 1. Creating a client with valid config
/// 2. Verifying registration in registry
/// 3. Destroying the client
/// 4. Verifying removal from registry
/// 5. Creating a new client
/// 6. Verifying new client ID
#[tokio::test]
async fn create_destroy_cycle() {
    // Get initial registry state
    let registry = paper_plane_adapter::global_registry();
    let initial_count = registry.count().await;

    // Step 1: Create client with valid config
    let config = ClientConfig {
        api_id: 12345,
        api_hash: "test_hash_integration".to_string(),
        database_path: "/tmp/tdlib_integration".to_string(),
        files_directory: "/tmp/tdlib_files_integration".to_string(),
        use_test_dc: false,
        default_dc_id: 2,
    };

    let client_id = RustgramClient::create(config)
        .await
        .expect("Failed to create client");

    // Step 2: Verify registration in registry
    assert!(registry.count().await >= initial_count + 1);
    assert!(registry.contains(client_id).await);

    // Step 3: Destroy client
    RustgramClient::destroy(client_id)
        .await
        .expect("Failed to destroy client");

    // Step 4: Verify removal from registry
    assert!(!registry.contains(client_id).await);

    // Step 5: Create new client
    let config2 = ClientConfig {
        api_id: 67890,
        api_hash: "test_hash_integration_2".to_string(),
        database_path: "/tmp/tdlib_integration_2".to_string(),
        files_directory: "/tmp/tdlib_files_integration_2".to_string(),
        use_test_dc: false,
        default_dc_id: 2,
    };

    let new_client_id = RustgramClient::create(config2)
        .await
        .expect("Failed to create second client");

    // Step 6: Verify new client ID (should be different from first)
    assert_ne!(client_id, new_client_id);
    assert!(new_client_id.is_valid());

    // Cleanup
    let _ = RustgramClient::destroy(new_client_id).await;
}

/// Integration test for multiple concurrent clients.
///
/// Verifies that multiple clients can be created and managed independently.
#[tokio::test]
async fn multiple_clients_concurrent() {
    let registry = paper_plane_adapter::global_registry();
    let initial_count = registry.count().await;

    let mut client_ids = Vec::new();

    // Create 5 clients
    for i in 0..5 {
        let config = ClientConfig {
            api_id: 20000 + i,
            api_hash: format!("concurrent_hash_{}", i),
            database_path: format!("/tmp/tdlib_concurrent_{}", i),
            files_directory: format!("/tmp/tdlib_files_concurrent_{}", i),
            use_test_dc: false,
            default_dc_id: 2,
        };

        let client_id = RustgramClient::create(config)
            .await
            .expect("Failed to create client");
        client_ids.push(client_id);
    }

    // Verify all are registered
    assert!(registry.count().await >= initial_count + 5);

    for id in &client_ids {
        assert!(registry.contains(*id).await);
    }

    // Destroy all clients
    for id in client_ids {
        RustgramClient::destroy(id)
            .await
            .expect("Failed to destroy client");
    }
}

/// Integration test for error handling.
///
/// Verifies proper error handling for invalid operations.
#[tokio::test]
async fn error_handling() {
    // Try to destroy non-existent client
    let fake_id = ClientId::new(999999);
    let result = RustgramClient::destroy(fake_id).await;
    assert!(result.is_err());

    // Try to create with invalid config
    let invalid_config = ClientConfig {
        api_id: -1,
        api_hash: String::new(),
        database_path: String::new(),
        files_directory: String::new(),
        use_test_dc: false,
        default_dc_id: 2,
    };

    let result = RustgramClient::create(invalid_config).await;
    assert!(result.is_err());
}
