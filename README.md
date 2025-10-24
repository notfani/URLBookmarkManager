# URLBookmarkManager
Небольшой проект реализованный в рамках предмета "Тестирование и верификация ПО"

## Стек технологий

- **Backend**: Rust (Actix Web, Diesel ORM)
- **Database**: PostgreSQL
- **Frontend**: React
- **Containerization**: Docker, Docker Compose

## Требования

- Docker
- Docker Compose

## Установка

1. Клонируйте репозиторий:
   ```bash
   git clone <repository-url>
   cd bookmark-manager
   ```

2. Соберите и запустите сервисы:
   ```bash
   docker-compose up --build
   ```

3. Приложение будет доступно по адресу `http://localhost:8080` (если используется Nginx) или `http://localhost:8000` (прямой доступ к API).

## Структура проекта

```
bookmark-manager/
├── backend/                 # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── models.rs        # Модели базы данных
│   │   ├── schema.rs        # Схема базы данных
│   │   ├── handlers.rs      # Обработчики API
│   │   └── db.rs            # Подключение к базе данных
│   ├── Cargo.toml
│   └── migrations/
├── frontend/                # React frontend
│   ├── public/
│   ├── src/
│   │   ├── components/
│   │   ├── App.js
│   │   └── index.js
│   └── package.json
├── nginx/                   # Конфигурация Nginx
│   └── nginx.conf
├── docker-compose.yml
├── Dockerfile.backend
├── Dockerfile.frontend
├── README.md
└── .env.example
```

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

### Закладки

- `GET /api/bookmarks` - Получить все закладки
- `POST /api/bookmarks` - Создать новую закладку
- `GET /api/bookmarks/{id}` - Получить конкретную закладку
- `PUT /api/bookmarks/{id}` - Обновить закладку
- `DELETE /api/bookmarks/{id}` - Удалить закладку
- `GET /api/bookmarks/search?q={query}` - Поиск закладок

### Категории

- `GET /api/categories` - Получить все категории
- `POST /api/categories` - Создать новую категорию

---

## Конфигурация Docker

### Сервисы

- **Backend**: Приложение на Rust
- **Frontend**: Приложение на React (собранное и обслуживаемое Nginx)
- **Database**: PostgreSQL
- **Nginx**: Обратный прокси (опционально)

### Переменные окружения

- `DATABASE_URL`: строка подключения к PostgreSQL
- `PORT`: порт сервера бэкенда
- `FRONTEND_PORT`: порт фронтенда

---

## Разработка

### Запуск в режиме разработки

1. Запустите базу данных:
   ```bash
   docker-compose up postgres
   ```

2. Запустите бэкенд:
   ```bash
   cd backend
   cargo run
   ```

3. Запустите фронтенд:
   ```bash
   cd frontend
   npm start
   ```

### Запуск тестов

Тесты бэкенда:
```bash
cd backend
cargo test
```

Тесты фронтенда:
```bash
cd frontend
npm test
```

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