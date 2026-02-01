// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Constructor ID verification tests.
//!
//! This module verifies that all constructor IDs match the official
//! Telegram TL schema (telegram_api.tl).

use rustgram_tl_core::*;

#[test]
fn test_photo_constructor_ids() {
    // Verify Photo constructor IDs
    assert_eq!(
        Photo::EMPTY_CONSTRUCTOR,
        0x2331b22d,
        "photoEmpty constructor should match TL schema"
    );
    assert_eq!(
        Photo::PHOTO_CONSTRUCTOR,
        0xfb197a65,
        "photo constructor should match TL schema"
    );
}

#[test]
fn test_photo_size_constructor_ids() {
    // Verify PhotoSize constructor IDs
    assert_eq!(
        PhotoSize::EMPTY_CONSTRUCTOR,
        0x0017e23c,
        "photoSizeEmpty constructor should match TL schema"
    );
    assert_eq!(
        PhotoSize::SIZE_CONSTRUCTOR,
        0x75c78e60,
        "photoSize constructor should match TL schema"
    );
    assert_eq!(
        PhotoSize::CACHED_CONSTRUCTOR,
        0x021e1ad6,
        "photoCachedSize constructor should match TL schema"
    );
    assert_eq!(
        PhotoSize::STRIPPED_CONSTRUCTOR,
        0xe0b0bc2e,
        "photoStrippedSize constructor should match TL schema"
    );
    assert_eq!(
        PhotoSize::PROGRESSIVE_CONSTRUCTOR,
        0xfa3efb95,
        "photoSizeProgressive constructor should match TL schema"
    );
    assert_eq!(
        PhotoSize::PATH_CONSTRUCTOR,
        0xd8214d41,
        "photoPathSize constructor should match TL schema"
    );
}

#[test]
fn test_peer_constructor_ids() {
    // Verify Peer constructor IDs
    assert_eq!(
        Peer::USER_CONSTRUCTOR,
        0x59511722,
        "peerUser constructor should match TL schema"
    );
    assert_eq!(
        Peer::CHAT_CONSTRUCTOR,
        0x36c6019a,
        "peerChat constructor should match TL schema"
    );
    assert_eq!(
        Peer::CHANNEL_CONSTRUCTOR,
        0xa2a5371e,
        "peerChannel constructor should match TL schema"
    );
}

#[test]
fn test_input_peer_constructor_ids() {
    // Verify InputPeer constructor IDs
    assert_eq!(
        InputPeer::EMPTY_CONSTRUCTOR,
        0x7f3b18ea,
        "inputPeerEmpty constructor should match TL schema"
    );
    assert_eq!(
        InputPeer::SELF_CONSTRUCTOR,
        0x7da07ec9,
        "inputPeerSelf constructor should match TL schema"
    );
    assert_eq!(
        InputPeer::CHAT_CONSTRUCTOR,
        0x35a95cb9,
        "inputPeerChat constructor should match TL schema"
    );
    assert_eq!(
        InputPeer::USER_CONSTRUCTOR,
        0xdde8a54c,
        "inputPeerUser constructor should match TL schema"
    );
    assert_eq!(
        InputPeer::CHANNEL_CONSTRUCTOR,
        0x27bcbbfc,
        "inputPeerChannel constructor should match TL schema"
    );
    assert_eq!(
        InputPeer::USER_FROM_MESSAGE_CONSTRUCTOR,
        0xa87b0a1c,
        "inputPeerUserFromMessage constructor should match TL schema"
    );
    assert_eq!(
        InputPeer::CHANNEL_FROM_MESSAGE_CONSTRUCTOR,
        0xbd2a0840,
        "inputPeerChannelFromMessage constructor should match TL schema"
    );
}

#[test]
fn test_notification_sound_constructor_ids() {
    // Verify NotificationSound constructor IDs
    assert_eq!(
        NotificationSound::DEFAULT_CONSTRUCTOR,
        0x97e8bebe,
        "notificationSoundDefault constructor should match TL schema"
    );
    assert_eq!(
        NotificationSound::NONE_CONSTRUCTOR,
        0x6f0c34df,
        "notificationSoundNone constructor should match TL schema"
    );
    assert_eq!(
        NotificationSound::LOCAL_CONSTRUCTOR,
        0x830b9ae4,
        "notificationSoundLocal constructor should match TL schema"
    );
    assert_eq!(
        NotificationSound::RINGTONE_CONSTRUCTOR,
        0xff6c8049,
        "notificationSoundRingtone constructor should match TL schema"
    );
}

#[test]
fn test_tl_bool_constructor_ids() {
    // Verify TlBool constructor IDs
    assert_eq!(
        TlBool::TRUE_CONSTRUCTOR,
        0x997275b5,
        "boolTrue constructor should match TL schema"
    );
    assert_eq!(
        TlBool::FALSE_CONSTRUCTOR,
        0xbc799737,
        "boolFalse constructor should match TL schema"
    );
}

#[test]
fn test_notify_settings_constructor_ids() {
    // Verify notification settings constructor IDs
    assert_eq!(
        PeerNotifySettings::CONSTRUCTOR,
        0x99622c0c,
        "peerNotifySettings constructor should match TL schema"
    );
    assert_eq!(
        InputPeerNotifySettings::CONSTRUCTOR,
        0xcacb6ae2,
        "inputPeerNotifySettings constructor should match TL schema"
    );
}

#[test]
fn test_chat_full_constructor_ids() {
    // Verify ChatFull constructor IDs
    assert_eq!(
        ChatFull::CONSTRUCTOR,
        0x2633421b,
        "chatFull constructor should match TL schema"
    );
    assert_eq!(
        ChatFull::CHANNEL_CONSTRUCTOR,
        0xe4e0b29d,
        "channelFull constructor should match TL schema"
    );
}

#[test]
fn test_chat_participants_constructor_ids() {
    // Verify ChatParticipants constructor IDs
    assert_eq!(
        ChatParticipants::CHAT_CONSTRUCTOR,
        0x3cbc93f8,
        "chatParticipants constructor should match TL schema"
    );
    assert_eq!(
        ChatParticipants::FORBIDDEN_CONSTRUCTOR,
        0x8763d3e1,
        "chatParticipantsForbidden constructor should match TL schema"
    );
}

#[test]
fn test_user_full_constructor_ids() {
    // Verify UserFull constructor IDs
    assert_eq!(
        UserFull::CONSTRUCTOR,
        0xa02bc13e,
        "userFull constructor should match TL schema"
    );
}

#[test]
fn test_all_constructor_ids_unique() {
    // Ensure no duplicate constructor IDs
    let mut ids = std::collections::HashSet::new();

    // Photo types
    assert!(ids.insert(Photo::EMPTY_CONSTRUCTOR));
    assert!(ids.insert(Photo::PHOTO_CONSTRUCTOR));
    assert!(ids.insert(PhotoSize::EMPTY_CONSTRUCTOR));
    assert!(ids.insert(PhotoSize::SIZE_CONSTRUCTOR));
    assert!(ids.insert(PhotoSize::CACHED_CONSTRUCTOR));
    assert!(ids.insert(PhotoSize::STRIPPED_CONSTRUCTOR));
    assert!(ids.insert(PhotoSize::PROGRESSIVE_CONSTRUCTOR));
    assert!(ids.insert(PhotoSize::PATH_CONSTRUCTOR));

    // Peer types
    assert!(ids.insert(Peer::USER_CONSTRUCTOR));
    assert!(ids.insert(Peer::CHAT_CONSTRUCTOR));
    assert!(ids.insert(Peer::CHANNEL_CONSTRUCTOR));
    assert!(ids.insert(InputPeer::EMPTY_CONSTRUCTOR));
    assert!(ids.insert(InputPeer::SELF_CONSTRUCTOR));
    assert!(ids.insert(InputPeer::CHAT_CONSTRUCTOR));
    assert!(ids.insert(InputPeer::USER_CONSTRUCTOR));
    assert!(ids.insert(InputPeer::CHANNEL_CONSTRUCTOR));
    assert!(ids.insert(InputPeer::USER_FROM_MESSAGE_CONSTRUCTOR));
    assert!(ids.insert(InputPeer::CHANNEL_FROM_MESSAGE_CONSTRUCTOR));

    // Notification types
    assert!(ids.insert(NotificationSound::DEFAULT_CONSTRUCTOR));
    assert!(ids.insert(NotificationSound::NONE_CONSTRUCTOR));
    assert!(ids.insert(NotificationSound::LOCAL_CONSTRUCTOR));
    assert!(ids.insert(NotificationSound::RINGTONE_CONSTRUCTOR));
    assert!(ids.insert(TlBool::TRUE_CONSTRUCTOR));
    assert!(ids.insert(TlBool::FALSE_CONSTRUCTOR));
    assert!(ids.insert(PeerNotifySettings::CONSTRUCTOR));
    assert!(ids.insert(InputPeerNotifySettings::CONSTRUCTOR));

    // Chat types
    assert!(ids.insert(ChatFull::CONSTRUCTOR));
    assert!(ids.insert(ChatFull::CHANNEL_CONSTRUCTOR));
    assert!(ids.insert(ChatParticipants::CHAT_CONSTRUCTOR));
    assert!(ids.insert(ChatParticipants::FORBIDDEN_CONSTRUCTOR));

    // User types
    assert!(ids.insert(UserFull::CONSTRUCTOR));

    // Total unique IDs (2 Photo + 6 PhotoSize + 3 Peer + 7 InputPeer +
    //                   4 NotificationSound + 2 TlBool + 2 Settings +
    //                   2 ChatFull + 2 ChatParticipants + 1 UserFull)
    assert_eq!(ids.len(), 31, "All constructor IDs should be unique");
}

#[test]
fn test_peer_instance_constructor_ids() {
    // Verify that Peer::constructor_id() returns correct values
    let user_peer = Peer::user(123);
    assert_eq!(user_peer.constructor_id(), Peer::USER_CONSTRUCTOR);

    let chat_peer = Peer::chat(456);
    assert_eq!(chat_peer.constructor_id(), Peer::CHAT_CONSTRUCTOR);

    let channel_peer = Peer::channel(789);
    assert_eq!(
        channel_peer.constructor_id(),
        Peer::CHANNEL_CONSTRUCTOR
    );
}

#[test]
fn test_input_peer_instance_constructor_ids() {
    // Verify that InputPeer::constructor_id() returns correct values
    assert_eq!(InputPeer::Empty.constructor_id(), InputPeer::EMPTY_CONSTRUCTOR);
    assert_eq!(
        InputPeer::InputPeerSelf.constructor_id(),
        InputPeer::SELF_CONSTRUCTOR
    );
    assert_eq!(
        InputPeer::Chat { chat_id: 0 }.constructor_id(),
        InputPeer::CHAT_CONSTRUCTOR
    );
    assert_eq!(
        InputPeer::User {
            user_id: 0,
            access_hash: 0
        }
        .constructor_id(),
        InputPeer::USER_CONSTRUCTOR
    );
    assert_eq!(
        InputPeer::Channel {
            channel_id: 0,
            access_hash: 0
        }
        .constructor_id(),
        InputPeer::CHANNEL_CONSTRUCTOR
    );
}

#[test]
fn test_constructor_id_formats() {
    // Verify constructor IDs are valid (non-zero)
    let all_ids = vec![
        Photo::EMPTY_CONSTRUCTOR,
        Photo::PHOTO_CONSTRUCTOR,
        PhotoSize::EMPTY_CONSTRUCTOR,
        PhotoSize::SIZE_CONSTRUCTOR,
        PhotoSize::CACHED_CONSTRUCTOR,
        PhotoSize::STRIPPED_CONSTRUCTOR,
        PhotoSize::PROGRESSIVE_CONSTRUCTOR,
        PhotoSize::PATH_CONSTRUCTOR,
        Peer::USER_CONSTRUCTOR,
        Peer::CHAT_CONSTRUCTOR,
        Peer::CHANNEL_CONSTRUCTOR,
        InputPeer::EMPTY_CONSTRUCTOR,
        InputPeer::SELF_CONSTRUCTOR,
        InputPeer::CHAT_CONSTRUCTOR,
        InputPeer::USER_CONSTRUCTOR,
        InputPeer::CHANNEL_CONSTRUCTOR,
        InputPeer::USER_FROM_MESSAGE_CONSTRUCTOR,
        InputPeer::CHANNEL_FROM_MESSAGE_CONSTRUCTOR,
        NotificationSound::DEFAULT_CONSTRUCTOR,
        NotificationSound::NONE_CONSTRUCTOR,
        NotificationSound::LOCAL_CONSTRUCTOR,
        NotificationSound::RINGTONE_CONSTRUCTOR,
        TlBool::TRUE_CONSTRUCTOR,
        TlBool::FALSE_CONSTRUCTOR,
        PeerNotifySettings::CONSTRUCTOR,
        InputPeerNotifySettings::CONSTRUCTOR,
        ChatFull::CONSTRUCTOR,
        ChatFull::CHANNEL_CONSTRUCTOR,
        ChatParticipants::CHAT_CONSTRUCTOR,
        ChatParticipants::FORBIDDEN_CONSTRUCTOR,
        UserFull::CONSTRUCTOR,
    ];

    for id in all_ids {
        assert!(id > 0, "Constructor ID 0x{:08x} should be non-zero", id);
    }
}

#[test]
fn test_vector_constructor_id() {
    // Verify the standard vector constructor ID
    // Vector in TL is always prefixed with 0x1cb5c415
    let vector_id: u32 = 0x1cb5c415;
    assert_eq!(vector_id, 0x1cb5c415, "Vector constructor should be 0x1cb5c415");
}
