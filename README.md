# URLBookmarkManager

Менеджер закладок с аутентификацией пользователей, реализованный в рамках предмета "Тестирование и верификация ПО".

🚀 **[Быстрый старт с автоматическими тестами →](QUICKSTART_TESTS.md)**  
🐛 **[Устранение неполадок →](TROUBLESHOOTING.md)**

## Стек технологий

- **Backend**: Rust (Actix Web 4.0, Diesel ORM 2.0)
- **Database**: PostgreSQL 14
- **Frontend**: Vanilla JavaScript (HTML5, CSS3)
- **Authentication**: JWT + BCrypt
- **Containerization**: Docker, Docker Compose
- **Web Server**: Nginx (reverse proxy)

## Требования

- Docker
- Docker Compose

## Установка и запуск

1. Клонируйте репозиторий:
   ```bash
   git clone <repository-url>
   cd URLBookmarkManager
   ```

2. Соберите и запустите все сервисы:
   ```bash
   docker-compose up --build
   ```

   ** Автоматический запуск тестов:** При первом запуске проекта автоматически выполнятся все тесты. Результаты будут выведены в консоль и сохранены в логах контейнера `bookmark_tests`.

3. Приложение будет доступно:
   - **Frontend (через Nginx)**: http://localhost:8080
   - **Backend API**: http://localhost:8000
   - **PostgreSQL**: localhost:5432

### Просмотр результатов тестов

После запуска проекта проверьте результаты автоматических тестов:

```bash
docker-compose logs test
```

## Структура проекта

```
URLBookmarkManager/
├── backend/                      # Rust backend
│   ├── src/
│   │   ├── main.rs              # Точка входа, настройка сервера
│   │   ├── model.rs             # Модели данных (Bookmark, Category, User)
│   │   ├── schema.rs            # Схема базы данных (Diesel)
│   │   ├── handlers.rs          # HTTP обработчики API
│   │   ├── db.rs                # Функции работы с БД
│   │   ├── auth.rs              # Аутентификация (JWT, BCrypt)
│   │   ├── config.rs            # Конфигурация из окружения
│   │   ├── middleware.rs        # Middleware для авторизации
│   │   └── tests/               # Модульные тесты
│   ├── Cargo.toml               # Зависимости Rust
│   ├── Dockerfile               # Docker образ для production
│   ├── Dockerfile.test          # Docker образ для тестов (с cargo)
│   └── migrations/              # Миграции базы данных
│       ├── 00000000000000_diesel_initial_setup/
│       ├── 00000000000001_create_tables/
│       └── 00000000000002_add_users/
├── database/                     # Конфигурация PostgreSQL
│   ├── Dockerfile
│   └── init-scripts/
│       └── 01-init.sql
├── frontend/                     # Vanilla JS frontend
│   ├── public/
│   │   ├── index.html           # Главная страница (список закладок)
│   │   ├── login.html           # Страница входа/регистрации
│   │   ├── app.js               # Основная логика приложения
│   │   ├── auth.js              # Логика аутентификации
│   │   └── style.css            # Стили приложения
│   ├── Dockerfile               # Docker образ для frontend
│   └── package.json
├── nginx/                        # Конфигурация Nginx
│   └── nginx.conf               # Reverse proxy настройки
├── docker-compose.yaml          # Оркестрация всех сервисов + автотесты
├── docker-compose.test.yaml     # Только БД + тесты (изолированно)
├── run_tests.ps1                # PowerShell скрипт запуска тестов
├── run_tests.bat                # CMD скрипт запуска тестов
├── LICENSE
├── README.md
├── TESTING.md                   # Документация по тестам
├── TEST_RESULTS.md              # Интерпретация результатов
├── QUICKSTART_TESTS.md          # Быстрый старт с тестами
├── COMMANDS.md                  # Справочник команд
└── CHANGELOG_TESTS.md           # История изменений
```

---

## Основные возможности

### Управление пользователями
- ✅ Регистрация с email и username
- ✅ Аутентификация по JWT токенам
- ✅ Хеширование паролей с BCrypt
- ✅ Получение информации о текущем пользователе

### Управление закладками
- ✅ Создание закладок с названием, URL и описанием
- ✅ Просмотр всех своих закладок
- ✅ Обновление существующих закладок
- ✅ Удаление закладок
- ✅ Поиск закладок по названию
- ✅ Привязка закладок к категориям
- ✅ Автоматическое отслеживание времени создания и обновления

### Управление категориями
- ✅ Создание категорий для организации закладок
- ✅ Просмотр всех категорий
- ✅ Фильтрация закладок по категориям
- ✅ Привязка категорий к пользователям

### Безопасность
- 🔒 JWT токены для авторизации
- 🔒 BCrypt для хеширования паролей
- 🔒 CORS настроен для работы с frontend
- 🔒 Middleware для проверки авторизации
- 🔒 Изоляция данных пользователей (каждый видит только свои закладки)

---

## Реализованные методологии тестирования

### TDD (Test-Driven Development)

Модульные и интеграционные тесты для операций бэкенда:

- Добавление закладок
- Получение закладок
- Обновление закладок
- Удаление закладок
- Управление категориями
- Функциональность поиска

Пример теста на Rust:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_bookmark() {
        // Реализация тестов
    }
}
```

### ATDD (Acceptance Test-Driven Development)

Сценарии для поиска и фильтрации:

- Поиск закладок по названию
- Фильтрация закладок по категории
- Комбинированный поиск и фильтрация
- Граничные случаи (пустой поиск, несуществующие категории)

### BDD (Behavior-Driven Development)

Сценарий: "Given закладка добавлена, When ищу по названию, Then закладка находится"

```gherkin
Функция: Поиск закладок
  Сценарий: Поиск по названию
    Дано закладка "Rust Programming" с URL "https://www.rust-lang.org" существует
    Когда я ищу "Rust Programming"
    Тогда закладка "Rust Programming" должна быть возвращена
```

### SDD (Specification by Example)

Таблицы данных для тестирования сценариев:

| Название          | URL                           | Категория   | Ожидаемый результат |
|-------------------|-------------------------------|-------------|----------------------|
| Rust Official     | https://www.rust-lang.org     | Programming | Найдена             |
| Google            | https://www.google.com        | Search      | Найдена             |
| Non-existent      | https://nonexistent.example   | Other       | Не найдена          |

---

## API Endpoints

### Аутентификация

- `POST /api/register` - Регистрация нового пользователя
  ```json
  {
    "username": "user",
    "email": "user@example.com",
    "password": "password123"
  }
  ```

- `POST /api/login` - Вход пользователя
  ```json
  {
    "username": "user",
    "password": "password123"
  }
  ```
  Возвращает JWT токен

- `GET /api/me` - Получить информацию о текущем пользователе (требует токен)

### Закладки

- `GET /api/bookmarks` - Получить все закладки текущего пользователя
- `POST /api/bookmarks` - Создать новую закладку (требует токен)
  ```json
  {
    "title": "Example",
    "url": "https://example.com",
    "description": "Description",
    "category_id": "uuid"
  }
  ```

- `GET /api/bookmarks/{id}` - Получить конкретную закладку
- `PUT /api/bookmarks/{id}` - Обновить закладку (требует токен)
- `DELETE /api/bookmarks/{id}` - Удалить закладку (требует токен)
- `GET /api/bookmarks/search?q={query}` - Поиск закладок по названию

### Категории

- `GET /api/categories` - Получить все категории пользователя
- `POST /api/categories` - Создать новую категорию (требует токен)
  ```json
  {
    "name": "Programming"
  }
  ```
- `GET /api/categories/{id}/bookmarks` - Получить все закладки в категории

---

## Конфигурация Docker

### Сервисы

1. **postgres** - База данных PostgreSQL 14
   - Порт: 5432
   - Имя БД: `bookmark_manager`
   - Пользователь: `postgres`
   - Пароль: `password`
   - Healthcheck для готовности

2. **backend** - Приложение на Rust (Actix Web)
   - Порт: 8000
   - Автоматически запускает миграции Diesel
   - Зависит от готовности postgres

3. **frontend** - Статические файлы (Vanilla JS)
   - Порт: 3000
   - Hot reload в режиме разработки

4. **nginx** - Обратный прокси
   - Порт: 8080
   - Проксирует запросы к backend и frontend

### Переменные окружения

#### Backend
- `DATABASE_URL`: `postgresql://postgres:password@postgres:5432/bookmark_manager`
- `SERVER_PORT`: `8000`
- `RUST_LOG`: `info`

#### Frontend
- `CHOKIDAR_USEPOLLING`: `true` (для hot reload в Docker)

### Volumes

- `postgres_data` - персистентное хранилище данных PostgreSQL

### Network

Все сервисы работают в общей сети `bookmark_network` с драйвером bridge.

---

## Разработка

### Запуск в режиме разработки

1. Запустите базу данных:
   ```bash
   docker-compose up postgres
   ```

2. Запустите бэкенд (локально):
   ```bash
   cd backend
   # Установите DATABASE_URL
   export DATABASE_URL=postgresql://postgres:password@localhost:5432/bookmark_manager
   # Выполните миграции
   diesel migration run
   # Запустите сервер
   cargo run
   ```

3. Откройте frontend:
   ```bash
   cd frontend/public
   # Используйте простой HTTP сервер, например:
   python -m http.server 3000
   # или
   npx serve
   ```

### Запуск тестов

📖 **Подробная документация по тестированию:** [TESTING.md](TESTING.md)

#### Вариант 1: Через Docker (рекомендуется для Windows)

Используйте готовый скрипт:

**PowerShell:**
```powershell
.\run_tests.ps1
```

**CMD:**
```cmd
run_tests.bat
```

Или вручную:
```bash
# Запустите базу данных
docker-compose up -d postgres

# Подождите 5 секунд для инициализации БД

# Запустите тесты
docker-compose run --rm -e DATABASE_URL=postgresql://postgres:password@postgres:5432/bookmark_test backend cargo test -- --nocapture
```

#### Вариант 2: Локально (требует MSVC Build Tools)

1. Установите [Visual Studio Build Tools 2022](https://visualstudio.microsoft.com/downloads/) с компонентом "Desktop development with C++"

2. Запустите базу данных:
```bash
docker-compose up -d postgres
```

3. Установите переменную окружения:
```bash
set DATABASE_URL=postgresql://postgres:password@localhost:5432/bookmark_test
```

4. Запустите тесты:
```bash
cd backend
cargo test
```

#### Вариант 3: Через WSL2 (если установлен)

```bash
wsl
cd /mnt/c/Users/notfani/RustroverProjects/URLBookmarkManager/backend
export DATABASE_URL=postgresql://postgres:password@localhost:5432/bookmark_test
cargo test
```

### Работа с миграциями

Создать новую миграцию:
```bash
cd backend
diesel migration generate <migration_name>
```

Применить миграции:
```bash
diesel migration run
```

Откатить последнюю миграцию:
```bash
diesel migration revert
```

---

## Технологии и зависимости

### Backend (Rust)

**Основные зависимости:**
- `actix-web = "4.0"` - Веб-фреймворк
- `actix-cors = "0.6"` - CORS middleware
- `diesel = "2.0"` - ORM с поддержкой PostgreSQL, UUID и Chrono
- `serde = "1.0"` - Сериализация/десериализация
- `uuid = "1.0"` - Генерация UUID
- `chrono = "0.4"` - Работа с датами
- `bcrypt = "0.15"` - Хеширование паролей
- `jsonwebtoken = "9.0"` - JWT токены
- `dotenv = "0.15"` - Загрузка переменных окружения
- `env_logger = "0.10"` - Логирование

**Тестовые зависимости:**
- `tokio = "1.0"` - Асинхронный рантайм для тестов

### Frontend

Чистый JavaScript (Vanilla JS) без зависимостей:
- HTML5
- CSS3
- Native Fetch API
- LocalStorage для хранения JWT

### База данных

- PostgreSQL 14
- Diesel CLI для миграций

---

## Развёртывание

Приложение контейнеризовано и может быть развернуто с использованием Docker Compose. Для продакшена рассмотрите:

- Использование переменных окружения для конфигурации
- Защиту базы данных
- Добавление логирования и мониторинга
- Использование конфигурации обратного прокси для продакшена

---

## Вклад в проект

1. Форкните репозиторий
2. Создайте ветку для новой функции
3. Внесите изменения и добавьте тесты
4. Отправьте pull request

---

## Лицензия

[MIT License](LICENSE)