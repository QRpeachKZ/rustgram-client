# Модуль rustgram-birthdate: Bit-packed хранение даты рождения

**Дата:** 2025-01-04
**Статус:** Выполнено

## Описание задачи

Реализовать модуль `rustgram-birthdate` для компактного хранения даты рождения пользователя в формате, совместимом с TDLib. Модуль использует bit-packing для хранения дня, месяца и года в одном `i32` значении.

## Выполненные работы

### Реализованные компоненты

**Изменённые файлы:**
- `crates/birthdate/Cargo.toml` — Манифест крейта
- `crates/birthdate/src/lib.rs` — Публичный API и документация
- `crates/birthdate/src/birthdate.rs` — Основной тип `Birthdate` с bit-packed хранением
- `crates/birthdate/src/error.rs` — Типы ошибок `BirthdateError`
- `crates/birthdate/src/tl.rs` — TL сериализация для Telegram API

## Формат хранения

### Bit-Packed Layout

```text
| Bits 31-9 | Bits 8-5 | Bits 4-0 |
|-----------|----------|----------|
| Year      | Month    | Day      |
| 23 bits   | 4 bits   | 5 bits   |
```

- **Day**: 5 bits (значения 1-31, 0 для пустого)
- **Month**: 4 bits (значения 1-12, 0 для пустого)
- **Year**: 23 bits (значения 1800-3000, или 0 для неизвестного)

## Детальное описание компонентов

### 1. Birthdate (birthdate.rs)

Основной тип для хранения даты рождения.

```rust
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Birthdate {
    birthdate: i32,  // Формат: day | (month << 5) | (year << 9)
}
```

#### Основные методы

| Метод | Описание |
|-------|----------|
| `new(day, month, year)` | Создание даты с валидацией |
| `day()` | Получить день (1-31) |
| `month()` | Получить месяц (1-12) |
| `year()` | Получить год (Option\<i32\>) |
| `is_empty()` | Проверка на пустую дату |
| `from_td_api(day, month, year)` | Создание из TD API |
| `to_td_api()` | Конвертация в TD API tuple |
| `from_telegram_api(birthday)` | Создание из Telegram API |
| `to_telegram_api()` | Конвертация в Telegram API |

#### Константы

```rust
pub const BIRTHDAY_MAGIC: u32 = 0x6c8e1e06;  // TL constructor ID
pub const YEAR_FLAG_MASK: u32 = 0x1;         // Флаг наличия года
```

#### Валидация

Модуль выполняет полную валидацию дат:

- День: 1-31 (или 0 только если месяц тоже 0)
- Месяц: 1-12 (или 0 только если день тоже 0)
- Год: 1800-3000 (или 0 для неизвестного)
- Комбинации: проверка корректности дня для месяца
- Високосные годы: 29 февраля допустимо только в високосные годы (или когда год неизвестен)

```rust
// Пример валидации февраля
let max_day = if month == 2 {
    if year == 0 || Self::is_leap_year(year) {
        29  // Високосный год или неизвестный
    } else {
        28  // Обычный год
    }
} else {
    i32::from(DAYS_IN_MONTH[month as usize])
};
```

### 2. BirthdateError (error.rs)

Типы ошибок для операций с датами рождения.

```rust
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BirthdateError {
    #[error("invalid day: {0}, must be between 1 and 31")]
    InvalidDay(i32),

    #[error("invalid month: {0}, must be between 1 and 12")]
    InvalidMonth(i32),

    #[error("invalid year: {0}, must be between 1800 and 3000, or 0 for unknown")]
    InvalidYear(i32),

    #[error("invalid date: {day}/{month}/{year}")]
    InvalidDate { day: i32, month: i32, year: i32 },

    #[error("invalid year flag in Telegram API data")]
    InvalidYearFlag,
}
```

### 3. TelegramApiBirthday (tl.rs)

Тип для TL сериализации, соответствующий схеме MTProto:

```text
birthday#6c8e1e06 flags:# day:int month:int year:flags.0?int = Birthday
```

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramApiBirthday {
    pub flags: u32,           // Бит 0 = год присутствует
    pub day: i32,             // 1-31
    pub month: i32,           // 1-12
    pub year: Option<i32>,    // 1800-3000
}
```

## Технические решения

### Bit Packing для компактности

**Проблема:** Хранение даты рождения требует минимального использования памяти.

**Решение:** Использовать битовые поля для упаковки в один `i32`:

```rust
let packed = day | (month << MONTH_SHIFT) | (year << YEAR_SHIFT);
```

**Причина выбора:**
- Совпадает с форматом TDLib для совместимости
- Минимальный размер (4 байта)
- Эффективное использование кэша процессора

### Валидация високосных лет

```rust
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
```

- 2000 — високосный (делится на 400)
- 1900 — НЕ високосный (делится на 100, но не на 400)
- 2024 — високосный (делится на 4, не на 100)

### Представление неизвестного года

Год 0 означает "неизвестный год", что позволяет хранить день и месяц без года:

```rust
pub fn year(&self) -> Option<i32> {
    let year = self.birthdate >> YEAR_SHIFT;
    if year == 0 { None } else { Some(year) }
}
```

## Модели данных

### Конвертация TD API

```rust
// Из TD API (day, month, year) -> Birthdate
let bd = Birthdate::from_td_api(15, 6, 1990).unwrap();

// В TD API
let (day, month, year) = bd.to_td_api();
// Returns: (15, 6, 1990)
```

### Конвертация Telegram API

```rust
// Из Telegram API
let telegram_bd = TelegramApiBirthday {
    flags: YEAR_FLAG_MASK,  // Год присутствует
    day: 15,
    month: 6,
    year: Some(1990),
};
let bd = Birthdate::from_telegram_api(&telegram_bd).unwrap();

// В Telegram API
let telegram_bd = bd.to_telegram_api();
```

### Display формат

```rust
// С годом
assert_eq!(bd.to_string(), "1990-06-15");

// Без года
assert_eq!(bd_no_year.to_string(), "----06-15");

// Пустая дата
assert_eq!(Birthdate::default().to_string(), "");
```

## Тестирование

- **Unit тесты:** 33
- **Doctest:** 12
- **Всего проверок:** 45+

### Категории тестов

| Категория | Тесты |
|-----------|-------|
| Валидация дат | day, month, year ranges |
| Високосные годы | 2000, 1900, 2024, 2023 |
| Февраль | 29 февраля validation |
| Пустые даты | all zeros |
| Конвертация TD API | roundtrip |
| Конвертация Telegram API | with/without year |
| Display | форматирование |
| Копирование | Copy, Clone traits |
| Граничные значения | min/max dates |

## Совместимость с TDLib

### Полное соответствие формату хранения

Сравнение с `references/td/td/telegram/Birthdate.h`:

| TDLib (C++) | rustgram-birthdate (Rust) |
|-------------|---------------------------|
| `int32 birthdate_` | `i32 birthdate` |
| `birthdate_ & 31` | `birthdate & DAY_MASK` |
| `(birthdate_ >> 5) & 15` | `(birthdate >> MONTH_SHIFT) & MONTH_MASK` |
| `birthdate_ >> 9` | `birthdate >> YEAR_SHIFT` |
| `is_empty()` | `is_empty()` |

### TL Schema соответствие

```
# TDLib TL schema
birthday#6c8e1e06 flags:# day:int month:int year:flags.0?int = Birthday

# rustgram-birthdate
BIRTHDAY_MAGIC = 0x6c8e1e06  # Совпадает
YEAR_FLAG_MASK = 0x1         # Совпадает
```

## Конфигурация

| Константа | Значение | Описание |
|-----------|----------|----------|
| `BIRTHDAY_MAGIC` | `0x6c8e1e06` | TL constructor ID |
| `YEAR_FLAG_MASK` | `0x1` | Бит флага наличия года |
| `DAY_MASK` | `0x1f` | Маска для дня (5 bits) |
| `MONTH_SHIFT` | `5` | Сдвиг для месяца |
| `MONTH_MASK` | `0xf` | Маска для месяца (4 bits) |
| `YEAR_SHIFT` | `9` | Сдвиг для года |
| `MIN_YEAR` | `1800` | Минимальный год |
| `MAX_YEAR` | `3000` | Максимальный год |

## Зависимости

```toml
[dependencies]
thiserror = "1.0"

[dev-dependencies]
# Нет дополнительных зависимостей для тестов
```

## Известные ограничения

### Диапазон годов

Годы ограничены диапазоном 1800-3000 из-за формата хранения (23 бита):

```rust
// 23 bits может хранить 0..8,388,607
// Для практических целей достаточно 1800-3000
const MIN_YEAR: i32 = 1800;
const MAX_YEAR: i32 = 3000;
```

Это совпадает с ограничениями TDLib.

## Как проверить

```bash
# 1. Сборка и проверка
cargo build -p rustgram-birthdate
cargo test -p rustgram-birthdate

# 2. Форматирование и линтеры
cargo fmt --check -p rustgram-birthdate
cargo clippy -p rustgram-birthdate

# 3. Все тесты
cargo test -p rustgram-birthdate --all-features
```

Ожидаемый результат:
- Все тесты проходят (45+ проверок)
- Нет warnings от clippy
- Форматирование соответствует rustfmt

### Пример использования

```rust
use rustgram_birthdate::Birthdate;

// Создание с годом
let bd = Birthdate::new(15, 6, 1990).unwrap();
assert_eq!(bd.day(), 15);
assert_eq!(bd.month(), 6);
assert_eq!(bd.year(), Some(1990));

// Создание без года (год неизвестен)
let bd_no_year = Birthdate::new(15, 6, 0).unwrap();
assert_eq!(bd_no_year.year(), None);

// Display
println!("{}", bd);  // "1990-06-15"
println!("{}", bd_no_year);  // "----06-15"

// Конвертация в Telegram API
let telegram_bd = bd.to_telegram_api();
assert_eq!(telegram_bd.flags, 1);  // YEAR_FLAG_MASK
```

---
*Документ создан: 2025-01-04*
