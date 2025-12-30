// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Integration tests for secret chat events

use rustgram_logevent::{
    CloseSecretChat, CreateSecretChat, EncryptedInputFile, EncryptedInputFileType,
    InboundSecretMessage, LogEvent, LogEventParser, LogEventStorerVec, OutboundSecretMessage,
    SecretChatEvent, SecretChatEventEnum, SecretChatEventType, TlParser, TlStorer,
};

#[test]
fn test_secret_chat_event_type_from_i32() {
    assert_eq!(
        SecretChatEventType::from_i32(1).unwrap(),
        SecretChatEventType::InboundSecretMessage
    );
    assert_eq!(
        SecretChatEventType::from_i32(2).unwrap(),
        SecretChatEventType::OutboundSecretMessage
    );
    assert_eq!(
        SecretChatEventType::from_i32(3).unwrap(),
        SecretChatEventType::CloseSecretChat
    );
    assert_eq!(
        SecretChatEventType::from_i32(4).unwrap(),
        SecretChatEventType::CreateSecretChat
    );
}

#[test]
fn test_secret_chat_event_type_invalid() {
    assert!(SecretChatEventType::from_i32(0).is_err());
    assert!(SecretChatEventType::from_i32(5).is_err());
    assert!(SecretChatEventType::from_i32(99).is_err());
    assert!(SecretChatEventType::from_i32(-1).is_err());
}

#[test]
fn test_encrypted_input_file_default() {
    let file = EncryptedInputFile::default();
    assert_eq!(file.magic, 0x4328_d38a);
    assert_eq!(file.type_, EncryptedInputFileType::Empty);
    assert_eq!(file.id, 0);
    assert_eq!(file.access_hash, 0);
    assert_eq!(file.parts, 0);
    assert_eq!(file.key_fingerprint, 0);
    assert!(file.is_empty());
}

#[test]
fn test_encrypted_input_file_new() {
    let file = EncryptedInputFile::new(EncryptedInputFileType::Uploaded, 12345, 67890, 3, 42);
    assert_eq!(file.magic, 0x4328_d38a);
    assert_eq!(file.type_, EncryptedInputFileType::Uploaded);
    assert_eq!(file.id, 12345);
    assert_eq!(file.access_hash, 67890);
    assert_eq!(file.parts, 3);
    assert_eq!(file.key_fingerprint, 42);
    assert!(!file.is_empty());
}

#[test]
fn test_encrypted_input_file_roundtrip() {
    let files = vec![
        EncryptedInputFile::default(),
        EncryptedInputFile::new(EncryptedInputFileType::Empty, 0, 0, 0, 0),
        EncryptedInputFile::new(EncryptedInputFileType::Uploaded, 12345, 67890, 3, 42),
        EncryptedInputFile::new(EncryptedInputFileType::BigUploaded, -1, -2, 5, 99),
        EncryptedInputFile::new(EncryptedInputFileType::Location, 111, 222, 0, 0),
    ];

    for file in files {
        let mut storer = LogEventStorerVec::new();
        file.store(&mut storer);
        let data = storer.into_inner();

        let mut parser = LogEventParser::new(&data);
        let parsed = EncryptedInputFile::parse(&mut parser).unwrap();
        parser.fetch_end().unwrap();

        assert_eq!(file, parsed);
    }
}

#[test]
fn test_inbound_secret_message_default() {
    let msg = InboundSecretMessage::default();
    assert_eq!(msg.log_event_id, 0);
    assert_eq!(msg.chat_id, 0);
    assert_eq!(msg.date, 0);
    assert_eq!(msg.auth_key_id, 0);
    assert_eq!(msg.message_id, 0);
    assert_eq!(msg.my_in_seq_no, -1);
    assert_eq!(msg.my_out_seq_no, -1);
    assert_eq!(msg.his_in_seq_no, -1);
    assert!(!msg.is_pending);
    assert!(!msg.has_encrypted_file);
}

#[test]
fn test_inbound_secret_message_new() {
    let msg = InboundSecretMessage::new(123, 456, 789, 101);
    assert_eq!(msg.chat_id, 123);
    assert_eq!(msg.date, 456);
    assert_eq!(msg.auth_key_id, 789);
    assert_eq!(msg.message_id, 101);
}

#[test]
fn test_inbound_secret_message_store() {
    let msg = InboundSecretMessage {
        log_event_id: 0,
        chat_id: 123,
        date: 456,
        auth_key_id: 789,
        message_id: 101,
        my_in_seq_no: 1,
        my_out_seq_no: 2,
        his_in_seq_no: 3,
        is_pending: true,
        has_encrypted_file: false,
        decrypted_message_layer: vec![1, 2, 3],
        encrypted_file: None,
    };

    let mut storer = LogEventStorerVec::new();
    msg.store(&mut storer);

    // Just verify it produces some output
    assert!(storer.len() > 0);
}

#[test]
fn test_outbound_secret_message_default() {
    let msg = OutboundSecretMessage::default();
    assert_eq!(msg.log_event_id, 0);
    assert_eq!(msg.chat_id, 0);
    assert_eq!(msg.random_id, 0);
    assert!(msg.encrypted_message.is_empty());
    assert!(msg.file.is_empty());
    assert_eq!(msg.message_id, 0);
    assert_eq!(msg.my_in_seq_no, -1);
    assert_eq!(msg.my_out_seq_no, -1);
    assert_eq!(msg.his_in_seq_no, -1);
    assert!(!msg.is_sent);
    assert!(!msg.need_notify_user);
    assert!(!msg.is_rewritable);
    assert!(!msg.is_external);
    assert!(!msg.is_silent);
    assert!(msg.action.is_none());
}

#[test]
fn test_outbound_secret_message_new() {
    let msg = OutboundSecretMessage::new(123, 456);
    assert_eq!(msg.chat_id, 123);
    assert_eq!(msg.random_id, 456);
}

#[test]
fn test_outbound_secret_message_roundtrip() {
    let msg = OutboundSecretMessage {
        log_event_id: 0,
        chat_id: 123,
        random_id: 456,
        encrypted_message: vec![1, 2, 3, 4],
        file: EncryptedInputFile::new(EncryptedInputFileType::Location, 111, 222, 0, 0),
        message_id: 789,
        my_in_seq_no: 1,
        my_out_seq_no: 2,
        his_in_seq_no: 3,
        is_sent: true,
        need_notify_user: false,
        is_rewritable: true,
        is_external: false,
        is_silent: true,
        action: None,
    };

    let mut storer = LogEventStorerVec::new();
    msg.store(&mut storer);
    let data = storer.into_inner();

    let mut parser = LogEventParser::new(&data);
    let parsed = OutboundSecretMessage::parse(&mut parser).unwrap();
    parser.fetch_end().unwrap();

    assert_eq!(msg.chat_id, parsed.chat_id);
    assert_eq!(msg.random_id, parsed.random_id);
    assert_eq!(msg.encrypted_message, parsed.encrypted_message);
    assert_eq!(msg.file, parsed.file);
    assert_eq!(msg.message_id, parsed.message_id);
    assert_eq!(msg.is_sent, parsed.is_sent);
    assert_eq!(msg.is_silent, parsed.is_silent);
}

#[test]
fn test_close_secret_chat_default() {
    let event = CloseSecretChat::default();
    assert_eq!(event.log_event_id, 0);
    assert_eq!(event.chat_id, 0);
    assert!(!event.delete_history);
    assert!(!event.is_already_discarded);
}

#[test]
fn test_close_secret_chat_new() {
    let event = CloseSecretChat::new(123);
    assert_eq!(event.chat_id, 123);
    assert!(!event.delete_history);
}

#[test]
fn test_close_secret_chat_with_flags() {
    let event = CloseSecretChat {
        log_event_id: 0,
        chat_id: 123,
        delete_history: true,
        is_already_discarded: true,
    };

    let mut storer = LogEventStorerVec::new();
    event.store(&mut storer);
    let data = storer.into_inner();

    let mut parser = LogEventParser::new(&data);
    let parsed = CloseSecretChat::parse(&mut parser).unwrap();
    parser.fetch_end().unwrap();

    assert_eq!(event.chat_id, parsed.chat_id);
    assert_eq!(event.delete_history, parsed.delete_history);
    assert_eq!(event.is_already_discarded, parsed.is_already_discarded);
}

#[test]
fn test_create_secret_chat_default() {
    let event = CreateSecretChat::default();
    assert_eq!(event.log_event_id, 0);
    assert_eq!(event.random_id, 0);
    assert_eq!(event.user_id, 0);
    assert_eq!(event.user_access_hash, 0);
}

#[test]
fn test_create_secret_chat_new() {
    let event = CreateSecretChat::new(123, 456, 789);
    assert_eq!(event.random_id, 123);
    assert_eq!(event.user_id, 456);
    assert_eq!(event.user_access_hash, 789);
}

#[test]
fn test_create_secret_chat_roundtrip() {
    let event = CreateSecretChat {
        log_event_id: 0,
        random_id: 123,
        user_id: 456,
        user_access_hash: 789,
    };

    let mut storer = LogEventStorerVec::new();
    event.store(&mut storer);
    let data = storer.into_inner();

    let mut parser = LogEventParser::new(&data);
    let parsed = CreateSecretChat::parse(&mut parser).unwrap();
    parser.fetch_end().unwrap();

    assert_eq!(event, parsed);
}

#[test]
fn test_secret_chat_event_enum_inbound() {
    let event =
        SecretChatEventEnum::InboundSecretMessage(InboundSecretMessage::new(123, 456, 789, 101));
    assert_eq!(
        event.event_type(),
        SecretChatEventType::InboundSecretMessage
    );
}

#[test]
fn test_secret_chat_event_enum_outbound() {
    let event = SecretChatEventEnum::OutboundSecretMessage(OutboundSecretMessage::new(123, 456));
    assert_eq!(
        event.event_type(),
        SecretChatEventType::OutboundSecretMessage
    );
}

#[test]
fn test_secret_chat_event_enum_close() {
    let event = SecretChatEventEnum::CloseSecretChat(CloseSecretChat::new(123));
    assert_eq!(event.event_type(), SecretChatEventType::CloseSecretChat);
}

#[test]
fn test_secret_chat_event_enum_create() {
    let event = SecretChatEventEnum::CreateSecretChat(CreateSecretChat::new(123, 456, 789));
    assert_eq!(event.event_type(), SecretChatEventType::CreateSecretChat);
}

#[test]
fn test_secret_chat_event_enum_store() {
    let event = SecretChatEventEnum::CreateSecretChat(CreateSecretChat::new(123, 456, 789));

    let mut storer = LogEventStorerVec::new();
    event.store(&mut storer);

    // Verify at least version + event_type + fields
    assert!(storer.len() >= 12);
}

#[test]
fn test_log_event_trait_inbound() {
    let mut msg = InboundSecretMessage::new(123, 456, 789, 101);
    assert_eq!(msg.log_event_id(), 0);

    msg.set_log_event_id(999);
    assert_eq!(msg.log_event_id(), 999);
}

#[test]
fn test_log_event_trait_outbound() {
    let mut msg = OutboundSecretMessage::new(123, 456);
    assert_eq!(msg.log_event_id(), 0);

    msg.set_log_event_id(888);
    assert_eq!(msg.log_event_id(), 888);
}

#[test]
fn test_log_event_trait_close() {
    let mut event = CloseSecretChat::new(123);
    assert_eq!(event.log_event_id(), 0);

    event.set_log_event_id(777);
    assert_eq!(event.log_event_id(), 777);
}

#[test]
fn test_log_event_trait_create() {
    let mut event = CreateSecretChat::new(123, 456, 789);
    assert_eq!(event.log_event_id(), 0);

    event.set_log_event_id(666);
    assert_eq!(event.log_event_id(), 666);
}

#[test]
fn test_log_event_trait_enum() {
    let mut event = SecretChatEventEnum::CreateSecretChat(CreateSecretChat::new(123, 456, 789));
    assert_eq!(event.log_event_id(), 0);

    event.set_log_event_id(555);
    assert_eq!(event.log_event_id(), 555);
}

#[test]
fn test_secret_chat_event_trait() {
    let msg = InboundSecretMessage::new(123, 456, 789, 101);
    assert_eq!(msg.event_type(), SecretChatEventType::InboundSecretMessage);

    let msg = OutboundSecretMessage::new(123, 456);
    assert_eq!(msg.event_type(), SecretChatEventType::OutboundSecretMessage);

    let event = CloseSecretChat::new(123);
    assert_eq!(event.event_type(), SecretChatEventType::CloseSecretChat);

    let event = CreateSecretChat::new(123, 456, 789);
    assert_eq!(event.event_type(), SecretChatEventType::CreateSecretChat);
}

#[test]
fn test_secret_chat_event_type_display() {
    assert_eq!(
        format!("{}", SecretChatEventType::InboundSecretMessage),
        "InboundSecretMessage"
    );
    assert_eq!(
        format!("{}", SecretChatEventType::OutboundSecretMessage),
        "OutboundSecretMessage"
    );
    assert_eq!(
        format!("{}", SecretChatEventType::CloseSecretChat),
        "CloseSecretChat"
    );
    assert_eq!(
        format!("{}", SecretChatEventType::CreateSecretChat),
        "CreateSecretChat"
    );
}

#[test]
fn test_encrypted_input_file_type_from_i32() {
    assert_eq!(
        EncryptedInputFileType::from_i32(0).unwrap(),
        EncryptedInputFileType::Empty
    );
    assert_eq!(
        EncryptedInputFileType::from_i32(1).unwrap(),
        EncryptedInputFileType::Uploaded
    );
    assert_eq!(
        EncryptedInputFileType::from_i32(2).unwrap(),
        EncryptedInputFileType::BigUploaded
    );
    assert_eq!(
        EncryptedInputFileType::from_i32(3).unwrap(),
        EncryptedInputFileType::Location
    );
}

#[test]
fn test_encrypted_input_file_type_invalid() {
    assert!(EncryptedInputFileType::from_i32(-1).is_err());
    assert!(EncryptedInputFileType::from_i32(4).is_err());
    assert!(EncryptedInputFileType::from_i32(99).is_err());
}
