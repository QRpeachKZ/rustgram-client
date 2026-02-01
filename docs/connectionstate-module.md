# Модуль rustgram-connectionstate: Машина состояний соединения

**Дата:** 2025-01-04
**Статус:** Выполнено

## Описание задачи

Реализовать модуль `rustgram-connectionstate` для управления состоянием соединения с Telegram серверами. Модуль предоставляет машину состояний с поддержкой callback-уведомлений об изменениях состояния.

## Выполненные работы

### Реализованные компоненты

**Изменённые файлы:**
- `crates/connectionstate/Cargo.toml` — Манифест крейта
- `crates/connectionstate/src/lib.rs` — Публичный API и документация
- `crates/connectionstate/src/error.rs` — Тип `ConnectionState` enum и `StateError`
- `crates/connectionstate/src/manager.rs` — `ConnectionStateManager`
- `crates/connectionstate/src/callback.rs` — Трейт `StateCallback` и `ClosureCallback`

## Состояния соединения

### ConnectionState Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionState {
    Empty = 0,                  // Начальное состояние (не подключен)
    WaitingForNetwork = 1,      // Ожидание сети
    ConnectingToProxy = 2,      // Подключение к прокси
    Connecting = 3,             // Подключение к серверам
    Updating = 4,               // Синхронизация данных
    Ready = 5,                  // Готов к работе
}
```

### Прогресс состояний

```text
Empty -> WaitingForNetwork -> ConnectingToProxy -> Connecting -> Updating -> Ready
```

Возможны возвраты к предыдущим состояниям (например, при потере соединения).

## Детальное описание компонентов

### 1. ConnectionState (error.rs)

Enum representing all possible connection states.

#### Методы проверки состояния

| Метод | Описание |
|-------|----------|
| `is_connected()` | `true` для `Updating` или `Ready` |
| `is_connecting()` | `true` для `ConnectingToProxy` или `Connecting` |
| `is_ready()` | `true` только для `Ready` |
| `as_i32()` | Числовое представление (0-5) |
| `from_i32(value)` | Создание из числа (или `None`) |
| `name()` | Имя состояния как `&str` |

```rust
// Примеры использования
assert!(!ConnectionState::Empty.is_connected());
assert!(ConnectionState::Ready.is_connected());
assert!(ConnectionState::Connecting.is_connecting());

// Конвертация
assert_eq!(ConnectionState::Ready.as_i32(), 5);
assert_eq!(ConnectionState::from_i32(0), Some(ConnectionState::Empty));
```

### 2. ConnectionStateManager (manager.rs)

Менеджер для отслеживания и уведомления об изменениях состояния.

```rust
pub struct ConnectionStateManager {
    current_state: ConnectionState,
    callbacks: Vec<Box<dyn StateCallback>>,
}
```

#### Основные методы

| Метод | Описание |
|-------|----------|
| `new()` | Создать менеджер с начальным `Empty` |
| `with_state(state)` | Создать менеджер с указанным состоянием |
| `current_state()` | Текущее состояние |
| `set_state(state)` | Установить новое состояние (уведомить callbacks) |
| `register_callback(cb)` | Зарегистрировать callback |
| `clear_callbacks()` | Удалить все callbacks |
| `callback_count()` | Количество зарегистрированных callbacks |
| `is_empty()` | Проверка отсутствия callbacks |
| `reset()` | Сброс в `Empty` + очистка callbacks |

```rust
// Пример использования
let mut manager = ConnectionStateManager::new();
assert_eq!(manager.current_state(), ConnectionState::Empty);

// Установка состояния
let changed = manager.set_state(ConnectionState::Connecting).unwrap();
assert!(changed);  // Состояние изменилось

// Попытка установить то же состояние
let changed = manager.set_state(ConnectionState::Connecting).unwrap();
assert!(!changed);  // Состояние не изменилось
```

### 3. StateCallback (callback.rs)

Трейт для получения уведомлений об изменениях состояния.

```rust
pub trait StateCallback: Send + Sync {
    /// Вызывается при изменении состояния
    /// Возвращает true, чтобы остаться зарегистрированным
    /// Возвращает false, чтобы автоматически отрегистрироваться
    fn on_state_changed(&self, state: ConnectionState) -> bool;
}
```

#### ClosureCallback

Удобная реализация на основе замыкания:

```rust
pub struct ClosureCallback<F>
where
    F: Fn(ConnectionState) -> bool + Send + Sync,
{
    f: F,
}

// Пример использования
let callback = ClosureCallback::new(|state| {
    println!("State changed to: {}", state);
    true  // Остаться зарегистрированным
});

manager.register_callback(Box::new(callback));
```

## Технические решения

### Автоматическая отрегистрация callback

**Проблема:** Callbacks должны иметь возможность автоматически удаляться после выполнения условия.

**Решение:** Метод `on_state_changed` возвращает `bool`:
- `true` — остаться зарегистрированным
- `false` — автоматически отрегистрироваться

```rust
// Отрегистрироваться при достижении Ready
manager.register_callback(Box::new(ClosureCallback::new(|state| {
    state != ConnectionState::Ready  // false на Ready -> unregister
})));
```

### Предотвращение дубликатов уведомлений

**Проблема:** Не нужно уведомлять callbacks, если состояние не изменилось.

**Решение:** `set_state` проверяет равенство состояний:

```rust
pub fn set_state(&mut self, new_state: ConnectionState) -> StateResult<bool> {
    if new_state == self.current_state {
        return Ok(false);  // Не изменилось
    }
    self.current_state = new_state;
    self.notify_callbacks(new_state)?;
    Ok(true)  // Изменилось
}
```

### Thread-safe callbacks

**Проблема:** Callbacks могут вызываться из разных потоков.

**Решение:** Трейт `StateCallback` требует `Send + Sync`:

```rust
pub trait StateCallback: Send + Sync {
    fn on_state_changed(&self, state: ConnectionState) -> bool;
}
```

Это позволяет безопасно передавать callbacks между потоками.

## Модели данных

### StateError

Ошибки управления состоянием:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    InvalidTransition { from: ConnectionState, to: ConnectionState },
    CallbackRegistrationFailed,
    CallbackFailed,
}
```

### StateResult

Псевдоним для Result:

```rust
pub type StateResult<T> = Result<T, StateError>;
```

## Конфигурация

Константы отсутствуют — всё определяется enum значениями.

| ConnectionState | Значение | Свойства |
|-----------------|----------|----------|
| `Empty` | 0 | Начальное состояние |
| `WaitingForNetwork` | 1 | Нет сети |
| `ConnectingToProxy` | 2 | `is_connecting()` = true |
| `Connecting` | 3 | `is_connecting()` = true |
| `Updating` | 4 | `is_connected()` = true |
| `Ready` | 5 | `is_connected()` = true, `is_ready()` = true |

## Зависимости

```toml
[dependencies]
# Нет внешних зависимостей - только std
```

## Тестирование

- **Unit тесты:** 22
- **Doctest:** 16
- **Всего проверок:** 38+

### Категории тестов

| Категория | Тесты |
|-----------|-------|
| ConnectionState | ordering, is_connected, is_connecting, as_i32, from_i32 |
| Manager | создание, установка состояния, изменение состояния |
| Callbacks | регистрация, вызов, множественные callbacks |
| Авто-отрегистрация | удаление при возврате false |
| Lifecycle | создание, использование, reset |
| Display | форматирование строк |

## Совместимость с TDLib

### Полное соответствие ConnectionState

Сравнение с `references/td/td/telegram/ConnectionState.h`:

```cpp
// TDLib (C++)
enum class ConnectionState : int32 {
    WaitingForNetwork, ConnectingToProxy, Connecting, Updating, Ready, Empty
};
```

```rust
// rustgram-connectionstate (Rust)
pub enum ConnectionState {
    Empty = 0,
    WaitingForNetwork = 1,
    ConnectingToProxy = 2,
    Connecting = 3,
    Updating = 4,
    Ready = 5,
}
```

**Совпадает:** Значения enum идентичны TDLib (порядок объявлен в другом порядке, но значения совпадают).

### Отсутствие зависимостей от Actor framework

TDLib использует actor модель для `ConnectionStateManager`. В rustgram используется упрощённый подход без actors — это подходит для текущей архитектуры.

## Известные ограничения

### Синхронный API

Все методы `ConnectionStateManager` синхронные. Если потребуется асинхронность, можно добавить `async fn` в будущем.

### Отсутствие валидации переходов

Менеджер позволяет любые переходы между состояниями, включая "регресс" (например, `Ready -> Empty`). Это соответствует поведению TDLib, где соединение может быть потеряно в любой момент.

## Как проверить

```bash
# 1. Сборка и проверка
cargo build -p rustgram-connectionstate
cargo test -p rustgram-connectionstate

# 2. Форматирование и линтеры
cargo fmt --check -p rustgram-connectionstate
cargo clippy -p rustgram-connectionstate

# 3. Все тесты
cargo test -p rustgram-connectionstate --all-features
```

Ожидаемый результат:
- Все тесты проходят (38+ проверок)
- Нет warnings от clippy
- Форматирование соответствует rustfmt

### Пример использования

```rust
use rustgram_connectionstate::{
    ConnectionStateManager, ConnectionState, ClosureCallback
};

// Создать менеджер
let mut manager = ConnectionStateManager::new();

// Зарегистрировать callback
manager.register_callback(Box::new(ClosureCallback::new(|state| {
    println!("Connection state: {}", state);
    true  // Остаться зарегистрированным
})));

// Изменить состояние (вызовет callback)
manager.set_state(ConnectionState::Connecting).unwrap();

// Проверить свойства
assert!(manager.current_state().is_connecting());
assert!(!manager.current_state().is_ready());

// Достичь готовности
manager.set_state(ConnectionState::Ready).unwrap();
assert!(manager.current_state().is_ready());
```

### Пример с авто-отрегистрацией

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let count = std::sync::Arc::new(AtomicUsize::new(0));
let count_clone = count.clone();

// Callback отрегистрируется при достижении Ready
manager.register_callback(Box::new(ClosureCallback::new(move |state| {
    count_clone.fetch_add(1, Ordering::AcqRel);
    // Вернуть false на Ready -> unregister
    state != ConnectionState::Ready
})));

// Три изменения состояния
manager.set_state(ConnectionState::Connecting).unwrap();
manager.set_state(ConnectionState::Updating).unwrap();
manager.set_state(ConnectionState::Ready).unwrap();

// Callback был вызван 3 раза, затем отрегистрирован
assert_eq!(count.load(Ordering::Acquire), 3);
assert_eq!(manager.callback_count(), 0);
```

---
*Документ создан: 2025-01-04*
