# Модуль rustgram-boost: Управление бустами чатов

**Дата:** 2025-01-04
**Статус:** Placeholder (ждёт MTProto transport)

## Описание задачи

Реализовать модуль `rustgram-boost` для управления chat boosts в Telegram. Бусты позволяют премиум-пользователям улучшать свои любимые чаты дополнительными возможностями.

## Выполненные работы

### Реализованные компоненты

**Изменённые файлы:**
- `crates/boost/Cargo.toml` — Манифест крейта
- `crates/boost/src/lib.rs` — Публичный API `BoostManager`
- `crates/boost/src/types.rs` — Типы данных для boost-сущностей
- `crates/boost/src/error.rs` — Типы ошибок `BoostError`
- `crates/boost/src/private.rs` — Внутренние функции (парсинг ссылок, валидация)

## Детальное описание компонентов

### 1. BoostManager (lib.rs)

Основной менеджер для управления бустами.

```rust
#[derive(Debug, Clone, Default)]
pub struct BoostManager {
    max_boost_level: i32,  // Максимальный уровень буста
}
```

#### Методы BoostManager

| Метод | Статус | Описание |
|-------|--------|----------|
| `new()` | Реализован | Создать менеджер (max_level = 10) |
| `with_max_level(level)` | Реализован | Создать с кастомным max level |
| `get_boost_slots()` | Placeholder | Получить доступные слоты |
| `get_dialog_boost_status(dialog_id)` | Placeholder | Статус бустов диалога |
| `boost_dialog(dialog_id, slot_ids)` | Placeholder | Применить бусты |
| `get_dialog_boost_link(dialog_id)` | Реализован | Получить boost link |
| `get_dialog_boost_link_info(url)` | Реализован | Парсить boost link |
| `get_dialog_boosts(dialog_id, ...)` | Placeholder | Получить бусты диалога |
| `get_user_dialog_boosts(dialog_id, user_id)` | Placeholder | Бусты от пользователя |
| `get_chat_boost_level_features(...)` | Реализован | Возможности уровня буста |

### 2. Boost link обработка

#### Формат boost links

```
# Публичный канал
https://t.me/boost/<username>
t.me/boost/<username>

# Приватный канал
https://t.me/boost?c=<channel_id>
```

#### Парсинг (parse_boost_link)

```rust
pub fn parse_boost_link(url: &str) -> Result<DialogBoostLinkInfo>
```

Поддерживает:
- Публичные ссылки с username
- Приватные ссылки с channel_id
- Короткий формат `t.me/...`

#### Форматирование (format_boost_link)

```rust
pub fn format_boost_link(
    username: Option<&str>,
    channel_id: ChannelId
) -> (String, bool)
```

Возвращает кортеж `(url, is_public)`.

### 3. Типы данных (types.rs)

#### DialogBoostLinkInfo

Информация о boost link для диалога:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DialogBoostLinkInfo {
    pub username: Option<String>,    // Для публичных
    pub channel_id: Option<ChannelId>, // Для приватных
}
```

#### ChatBoostSource

Источник буста:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ChatBoostSource {
    Premium { user_id: UserId },
    GiftCode { user_id: UserId, gift_slug: Option<String> },
    Giveaway {
        user_id: Option<UserId>,
        gift_slug: Option<String>,
        stars: Option<i64>,
        giveaway_message_id: i32,
        unclaimed: bool,
    },
}
```

#### ChatBoost

Информация о бусте:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoost {
    pub id: String,
    pub multiplier: i32,
    pub source: ChatBoostSource,
    pub date: i32,              // Unix timestamp
    pub expiration_date: i32,   // Unix timestamp
}
```

#### ChatBoostSlot

Слот буста пользователя:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSlot {
    pub slot_id: i32,
    pub dialog_id: Option<DialogId>,
    pub start_date: i32,
    pub expiration_date: i32,
    pub cooldown_until_date: i32,
}
```

Методы:
- `unused(slot_id, expiration_date)` — создать неиспользуемый слот
- `active(...)` — создать активный слот
- `is_used()` — проверка использования
- `is_on_cooldown(current_time)` — проверка cooldown

#### ChatBoostStatus

Статус бустов чата:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostStatus {
    pub boost_url: Option<String>,
    pub my_boost_slots: Vec<i32>,
    pub level: i32,
    pub gift_code_boost_count: i32,
    pub boost_count: i32,
    pub current_level_boost_count: i32,
    pub next_level_boost_count: i32,
    pub premium_member_count: i32,
    pub premium_member_percentage: f64,
    pub prepaid_giveaways: Vec<PrepaidGiveaway>,
}
```

#### ChatBoostLevelFeatures

Возможности на уровне буста:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostLevelFeatures {
    pub level: i32,                      // Запрошенный уровень
    pub actual_level: i32,               // Фактический (clamped)
    pub profile_accent_color_count: i32,
    pub title_color_count: i32,
    pub can_set_profile_background_custom_emoji: bool,
    pub accent_color_count: i32,
    pub can_set_background_custom_emoji: bool,
    pub can_set_emoji_status: bool,
    pub chat_theme_count: i32,
    pub can_set_custom_background: bool,
    pub can_set_custom_emoji_sticker_set: bool,
    pub can_enable_autotranslation: bool,
    pub can_recognize_speech: bool,
    pub can_restrict_sponsored_messages: bool,
}
```

### 4. BoostError (error.rs)

Ошибки операций с бустами:

```rust
#[derive(Error, Debug)]
pub enum BoostError {
    #[error("Invalid dialog: {0}")]
    InvalidDialog(String),

    #[error("Invalid user ID: {0}")]
    InvalidUserId(String),

    #[error("Invalid slot ID: {0}")]
    InvalidSlotId(String),

    #[error("Access denied to dialog: {0}")]
    AccessDenied(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid boost link: {0}")]
    InvalidBoostLink(String),

    #[error("Limit must be positive, got {0}")]
    InvalidLimit(i32),

    #[error("Cannot boost this type of chat")]
    CannotBoostChat,

    #[error("Internal error: {0}")]
    Internal(String),
}
```

## Технические решения

### Уровни бустов и возможности

Модуль реализует таблицу возможностей для уровней 0-10:

| Уровень | Возможности |
|---------|-------------|
| 0-1 | Базовые функции |
| 2 | `can_set_emoji_status` |
| 3 | Accent colors, title colors, profile accent colors |
| 4 | `can_set_background_custom_emoji` |
| 5 | `can_set_profile_background_custom_emoji`, chat themes |
| 7 | `can_set_custom_background` |
| 8 | `can_set_custom_emoji_sticker_set` |
| 9 | `can_enable_autotranslation`, `can_recognize_speech` |
| 10 | `can_restrict_sponsored_messages` |

```rust
pub fn get_chat_boost_level_features(
    &self,
    for_megagroup: bool,
    level: i32
) -> ChatBoostLevelFeatures {
    let actual_level = if level < 0 { 0 } else { level.min(self.max_boost_level) };

    ChatBoostLevelFeatures {
        level,
        actual_level,
        can_set_emoji_status: actual_level >= 2,
        can_set_background_custom_emoji: actual_level >= 4,
        can_set_profile_background_custom_emoji: actual_level >= 5,
        can_enable_autotranslation: actual_level >= 9,
        can_restrict_sponsored_messages: actual_level >= 10,
        // ...
    }
}
```

### Валидация входных данных

```rust
// Валидация slot IDs (должны быть неотрицательными)
pub fn validate_slot_ids(slot_ids: &[i32]) -> Result<()> {
    for &slot_id in slot_ids {
        if slot_id < 0 {
            return Err(BoostError::InvalidSlotId(...));
        }
    }
    Ok(())
}

// Валидация limit (должен быть положительным)
pub fn validate_limit(limit: i32) -> Result<()> {
    if limit <= 0 {
        return Err(BoostError::InvalidLimit(limit));
    }
    Ok(())
}
```

### Обработка boost links

Поддерживаются два формата ссылок:

1. **Публичные**: `https://t.me/boost/<username>`
   - Используется для каналов с публичным username

2. **Приватные**: `https://t.me/boost?c=<channel_id>`
   - Используется для приватных каналов

```rust
pub fn parse_boost_link(url: &str) -> Result<DialogBoostLinkInfo> {
    // Проверка префикса
    if !url.starts_with("https://t.me/boost") && !url.starts_with("t.me/boost") {
        return Err(BoostError::InvalidBoostLink(...));
    }

    // Извлечение username или channel_id
    // ...
}
```

## Модели данных

### Пример использования типов

```rust
use rustgram_boost::{BoostManager, ChatBoostSlot, DialogBoostLinkInfo};

// Создать менеджер
let manager = BoostManager::new();

// Получить возможности уровня 5
let features = manager.get_chat_boost_level_features(false, 5);
assert!(features.can_set_emoji_status);
assert!(features.can_set_profile_background_custom_emoji);

// Парсить boost link
let info = manager.get_dialog_boost_link_info("https://t.me/boost/mychannel")?;
assert!(info.is_public());
assert_eq!(info.username, Some("mychannel".to_string()));

// Создать слот буста
let slot = ChatBoostSlot::unused(0, 1735689600);
assert!(!slot.is_used());
```

## Конфигурация

| Параметр | По умолчанию | Описание |
|----------|-------------|----------|
| `max_boost_level` | `10` | Максимальный уровень буста |

## Зависимости

```toml
[dependencies]
rustgram-types = { path = "../types" }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
```

## Тестирование

- **Unit тесты:** 30
- **Doctest:** 10
- **Всего проверок:** 40+

### Категории тестов

| Категория | Тесты |
|-----------|-------|
| BoostManager | создание, max level |
| Boost links | публичные, приватные, невалидные |
| DialogBoostLinkInfo | public(), private(), is_public() |
| ChatBoost | is_expired() |
| ChatBoostSlot | unused, active, is_used, is_on_cooldown |
| ChatBoostSlots | available_count |
| Валидация | slot_ids, limit |
| Уровни бустов | level 0, 5, >max, negative |

## Совместимость с TDLib

### Соответствие BoostManager

Сравнение с `references/td/td/telegram/BoostManager.h`:

| TDLib метод | rustgram-boost метод | Статус |
|-------------|---------------------|--------|
| `get_boost_slots()` | `get_boost_slots()` | Placeholder |
| `get_dialog_boost_status()` | `get_dialog_boost_status()` | Placeholder |
| `boost_dialog()` | `boost_dialog()` | Placeholder |
| `get_dialog_boost_link()` | `get_dialog_boost_link()` | Реализован |
| `get_dialog_boost_link_info()` | `get_dialog_boost_link_info()` | Реализован |
| `get_dialog_boosts()` | `get_dialog_boosts()` | Placeholder |
| `get_user_dialog_boosts()` | `get_user_dialog_boosts()` | Placeholder |
| `get_chat_boost_level_features()` | `get_chat_boost_level_features()` | Реализован |

### Соответствие DialogBoostLinkInfo

```cpp
// TDLib (C++)
struct DialogBoostLinkInfo {
  string username;
  ChannelId channel_id;
};
```

```rust
// rustgram-boost
pub struct DialogBoostLinkInfo {
    pub username: Option<String>,
    pub channel_id: Option<ChannelId>,
}
```

**Отличие:** Используется `Option` для указания того, какое поле присутствует.

## Известные ограничения

### Placeholder реализация

Большинство методов возвращают `BoostError::NetworkError`:

```rust
pub fn get_boost_slots(&self) -> Result<ChatBoostSlots> {
    Err(BoostError::network(
        "Network operations not yet implemented"
    ))
}
```

### Отсутствие интеграции с ChatManager

Метод `get_dialog_boost_link` пока не может получить username канала:

```rust
// TODO: Get username from ChatManager when available
// For now, return private link
let (url, is_public) = format_boost_link(None, channel_id);
```

### Отсутствие MTProto transport

Все сетевые операции невозможны без реализованного MTProto transport слоя.

## TODO

- [ ] Реализовать сетевые методы после готовности MTProto transport
- [ ] Интегрироваться с ChatManager для получения username
- [ ] Добавить кэширование boost status
- [ ] Добавить автоматическое обновление истекающих бустов
- [ ] Добавить поддержку giveaways

## Как проверить

```bash
# 1. Сборка и проверка
cargo build -p rustgram-boost
cargo test -p rustgram-boost

# 2. Форматирование и линтеры
cargo fmt --check -p rustgram-boost
cargo clippy -p rustgram-boost

# 3. Все тесты
cargo test -p rustgram-boost --all-features
```

Ожидаемый результат:
- Все тесты проходят (40+ проверок)
- Нет warnings от clippy
- Форматирование соответствует rustfmt

### Пример использования

```rust
use rustgram_boost::BoostManager;

// Создать менеджер
let manager = BoostManager::new();

// Получить возможности уровня
let features = manager.get_chat_boost_level_features(false, 5);
println!("Level 5 features:");
println!("  Can set emoji status: {}", features.can_set_emoji_status);
println!("  Can set custom background: {}", features.can_set_custom_background);

// Парсить boost link
let info = manager.get_dialog_boost_link_info("https://t.me/boost/mychannel")?;
if info.is_public() {
    println!("Public channel: {}", info.username.unwrap());
} else {
    println!("Private channel: {:?}", info.channel_id);
}

// Получить boost link для диалога
use rustgram_types::{DialogId, ChannelId};

let channel_id = ChannelId::try_from(12345).unwrap();
let dialog_id = DialogId::from_channel(channel_id);
let (link, is_public) = manager.get_dialog_boost_link(dialog_id)?;
println!("Boost link: {} (public: {})", link, is_public);
```

---
*Документ создан: 2025-01-04*
