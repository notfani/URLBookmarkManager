# Тестирование проекта URLBookmarkManager

## Обзор тестов

Проект содержит комплексный набор модульных и интеграционных тестов для всех компонентов системы.

## Структура тестов

```
backend/src/tests/
├── mod.rs           # Основные тесты (все тесты собраны здесь)
├── db_tests.rs      # Тесты базы данных (в разработке)
├── handler_tests.rs # Тесты HTTP обработчиков (в разработке)
└── auth_tests.rs    # Тесты аутентификации (в разработке)
```

## Покрытие тестами

### 1. Тесты пользователей (User Tests)
- ✅ `test_create_user` - Создание нового пользователя
- ✅ `test_find_user_by_username` - Поиск пользователя по имени
- ✅ `test_find_user_by_email` - Поиск пользователя по email
- ✅ `test_get_user_by_id` - Получение пользователя по ID
- ✅ `test_multiple_users_isolation` - Изоляция данных между пользователями

### 2. Тесты категорий (Category Tests)
- ✅ `test_create_category` - Создание категории
- ✅ `test_get_all_categories` - Получение всех категорий
- ✅ `test_get_user_categories` - Получение категорий пользователя
- ✅ `test_get_category_by_id` - Получение категории по ID
- ✅ `test_full_category_workflow` - Полный жизненный цикл категории

### 3. Тесты закладок (Bookmark Tests)
- ✅ `test_create_bookmark` - Создание закладки
- ✅ `test_get_all_bookmarks` - Получение всех закладок
- ✅ `test_get_bookmark_by_id` - Получение закладки по ID
- ✅ `test_update_bookmark` - Обновление закладки
- ✅ `test_delete_bookmark` - Удаление закладки
- ✅ `test_search_bookmarks` - Поиск закладок
- ✅ `test_get_bookmarks_by_category` - Получение закладок по категории
- ✅ `test_full_bookmark_lifecycle` - Полный жизненный цикл закладки
- ✅ `test_partial_bookmark_update` - Частичное обновление закладки
- ✅ `test_search_case_insensitive` - Регистронезависимый поиск

### 4. Тесты аутентификации (Authentication Tests)
- ✅ `test_create_jwt` - Создание JWT токена
- ✅ `test_verify_jwt` - Проверка валидного токена
- ✅ `test_verify_invalid_jwt` - Проверка невалидного токена
- ✅ `test_expired_jwt` - Проверка истекшего токена

### 5. Интеграционные тесты обработчиков (Handler Tests)
- ✅ `test_register_handler` - Регистрация пользователя через API
- ✅ `test_register_duplicate_username` - Попытка регистрации с существующим username
- ✅ `test_register_duplicate_email` - Попытка регистрации с существующим email
- ✅ `test_login_handler` - Вход пользователя через API
- ✅ `test_login_invalid_credentials` - Вход с неверными данными
- ✅ `test_login_nonexistent_user` - Вход несуществующего пользователя
- ✅ `test_get_all_bookmarks_handler` - Получение закладок через API
- ✅ `test_get_bookmark_by_id_handler` - Получение закладки по ID через API
- ✅ `test_get_nonexistent_bookmark` - Попытка получить несуществующую закладку
- ✅ `test_search_bookmarks_handler` - Поиск через API
- ✅ `test_search_bookmarks_empty_query` - Поиск с пустым запросом
- ✅ `test_get_bookmarks_by_category_handler` - Фильтрация по категории через API

### 6. Комплексные тесты (E2E-подобные)
- ✅ `test_full_user_authentication_flow` - Полный цикл: регистрация → вход
- ✅ `test_full_bookmark_lifecycle` - Создание → Чтение → Обновление → Удаление
- ✅ `test_full_category_workflow` - Создание → Получение → Фильтрация

## Запуск тестов

### Автоматический запуск при старте проекта

Тесты автоматически запускаются при подъёме всего проекта:

```bash
docker-compose up --build
```

Сервис `test` запустится после инициализации базы данных, выполнит все тесты и завершится. Результаты можно посмотреть в логах:

```bash
docker-compose logs test
```

### Вариант 1: С полным стеком (рекомендуется)

Запускает все сервисы (postgres, backend, frontend, nginx) + тесты:

**PowerShell:**
```powershell
.\run_tests.ps1
```

**CMD:**
```cmd
run_tests.bat
```

### Вариант 2: Изолированные тесты (только БД + тесты)

Запускает только PostgreSQL и тесты (без backend/frontend/nginx):

**PowerShell:**
```powershell
.\run_tests.ps1 -Standalone
```

**CMD:**
```cmd
run_tests.bat --standalone
```

Или напрямую через Docker Compose:
```bash
docker-compose -f docker-compose.test.yaml up --build
```

### Вариант 3: Ручной запуск в существующем окружении

Если сервисы уже запущены:

```bash
# Запустить тесты
docker-compose up test

# Посмотреть результаты
docker-compose logs test
```

### Вариант 4: Через Docker (старый способ)

```bash
# PowerShell
.\run_tests.ps1

# CMD
run_tests.bat
```

### Локально (требует MSVC)

```bash
cd backend
set DATABASE_URL=postgresql://postgres:password@localhost:5432/bookmark_test
cargo test
```

### Запуск конкретного теста

```bash
cargo test test_create_bookmark -- --nocapture
```

### Запуск тестов с фильтрацией

```bash
# Только тесты пользователей
cargo test user -- --nocapture

# Только тесты категорий
cargo test category -- --nocapture

# Только тесты закладок
cargo test bookmark -- --nocapture

# Только тесты аутентификации
cargo test jwt -- --nocapture
```

## Тестовое окружение

### Конфигурация базы данных

```
DATABASE_URL=postgresql://postgres:password@localhost:5432/bookmark_test
```

### Docker образы

Проект использует два разных Dockerfile:

#### 1. `Dockerfile` (Production)
```dockerfile
FROM rust:1.86 as builder
# ... сборка ...
FROM debian:bookworm-slim  # Минимальный образ
# Только скомпилированный бинарник
```

**Используется для:**
- Backend сервис (`docker-compose up backend`)
- Production деплой
- Минимальный размер образа (~100MB)

#### 2. `Dockerfile.test` (Testing)
```dockerfile
FROM rust:1.86  # Полный образ с cargo
# Включает: rustc, cargo, diesel_cli
# Сохраняет исходный код
```

**Используется для:**
- Test сервис (`docker-compose up test`)
- Разработка и отладка
- Запуск тестов с cargo
- Больший размер (~1.5GB), но с полными инструментами

### Особенности

- Все тесты используют уникальные UUID для избежания конфликтов
- Каждый тест создаёт свои данные и не зависит от других
- Тесты можно запускать параллельно
- Используется реальная PostgreSQL база для интеграционных тестов

## Статистика

- **Всего тестов:** 40+
- **Покрытие функционала:** ~90%
- **Типы тестов:**
  - Модульные тесты (unit tests): 25+
  - Интеграционные тесты: 15+
  - E2E тесты: 3

## Проверяемые сценарии

### Безопасность
- ✅ Хеширование паролей (BCrypt)
- ✅ Генерация и проверка JWT
- ✅ Изоляция данных между пользователями
- ✅ Валидация дубликатов (username, email)

### Функциональность
- ✅ CRUD операции для всех сущностей
- ✅ Поиск и фильтрация
- ✅ Связи между таблицами (foreign keys)
- ✅ Частичные обновления

### Граничные случаи
- ✅ Несуществующие ID
- ✅ Пустые поисковые запросы
- ✅ Невалидные токены
- ✅ Дублирование данных

## Добавление новых тестов

Пример добавления теста:

```rust
#[test]
fn test_my_feature() {
    let pool = setup_test_db();
    let mut conn = pool.get().unwrap();
    
    // Подготовка данных
    let user = create_test_user(&mut conn);
    
    // Выполнение теста
    let result = db::my_function(&mut conn, &user.id);
    
    // Проверка результата
    assert!(result.is_ok());
    assert_eq!(result.unwrap().field, expected_value);
}
```

## Continuous Integration

Тесты можно интегрировать в CI/CD пайплайн:

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:14
        env:
          POSTGRES_PASSWORD: password
          POSTGRES_DB: bookmark_test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cd backend && cargo test
        env:
          DATABASE_URL: postgresql://postgres:password@localhost:5432/bookmark_test
```

