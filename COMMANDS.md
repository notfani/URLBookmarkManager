# Краткая справка по командам

## Запуск проекта

### Полный запуск всех сервисов
```bash
docker-compose up --build
```

### Запуск отдельных сервисов
```bash
docker-compose up -d postgres    # Только база данных
docker-compose up -d backend     # Бэкенд
docker-compose up -d frontend    # Фронтенд
docker-compose up -d nginx       # Nginx
```

### Остановка
```bash
docker-compose down              # Остановить все сервисы
docker-compose down -v           # + удалить volumes
```

## Тестирование

### Автоматический запуск тестов
```bash
# При старте всего проекта тесты запускаются автоматически
docker-compose up --build

# Просмотр результатов автоматических тестов
docker-compose logs test
```

### Запуск всех тестов (с полным стеком)
```powershell
# PowerShell
.\run_tests.ps1

# CMD
run_tests.bat
```

### Изолированный запуск тестов (только БД + тесты)
```powershell
# PowerShell
.\run_tests.ps1 -Standalone

# CMD
run_tests.bat --standalone

# Или напрямую
docker-compose -f docker-compose.test.yaml up --build
```

### Запуск тестов в работающем окружении
```bash
# Если сервисы уже запущены
docker-compose up test

# Перезапустить тесты
docker-compose restart test
docker-compose logs -f test
```

### Запуск тестов через Docker вручную
```bash
docker-compose up -d postgres
docker-compose run --rm backend cargo test -- --nocapture
```

### Запуск конкретного теста
```bash
docker-compose run --rm backend cargo test test_create_bookmark -- --nocapture
```

### Запуск тестов с фильтром
```bash
# Только тесты пользователей
docker-compose run --rm backend cargo test user -- --nocapture

# Только тесты аутентификации
docker-compose run --rm backend cargo test auth -- --nocapture
```

## Миграции базы данных

### Применить миграции
```bash
docker-compose run --rm backend diesel migration run
```

### Создать новую миграцию
```bash
docker-compose run --rm backend diesel migration generate <migration_name>
```

### Откатить последнюю миграцию
```bash
docker-compose run --rm backend diesel migration revert
```

## Разработка

### Просмотр логов
```bash
docker-compose logs -f backend   # Логи бэкенда
docker-compose logs -f postgres  # Логи БД
docker-compose logs -f           # Все логи
```

### Перезапуск сервиса
```bash
docker-compose restart backend
docker-compose restart frontend
```

### Подключение к контейнеру
```bash
docker-compose exec backend sh   # Shell в backend
docker-compose exec postgres psql -U postgres -d bookmark_manager  # PostgreSQL CLI
```

### Пересборка после изменений
```bash
docker-compose up --build backend    # Пересобрать бэкенд
docker-compose up --build frontend   # Пересобрать фронтенд
```

## Отладка

### Проверка статуса контейнеров
```bash
docker-compose ps
```

### Просмотр использования ресурсов
```bash
docker stats
```

### Очистка неиспользуемых ресурсов
```bash
docker system prune -a     # ОСТОРОЖНО: удаляет все неиспользуемые образы
```

### Проверка подключения к БД
```bash
docker-compose exec postgres psql -U postgres -c "SELECT version();"
```

## Полезные URL

- Frontend: http://localhost:8080
- Backend API: http://localhost:8000/api
- PostgreSQL: localhost:5432
- Документация API: http://localhost:8000/api/docs (если добавите Swagger)

## Переменные окружения

### Backend
```bash
DATABASE_URL=postgresql://postgres:password@postgres:5432/bookmark_manager
SERVER_PORT=8000
RUST_LOG=info
JWT_SECRET=your-secret-key-here
```

### Frontend
```bash
CHOKIDAR_USEPOLLING=true
```

## Быстрые проверки

### Проверка работы API
```bash
# Health check (если реализован)
curl http://localhost:8000/health

# Регистрация
curl -X POST http://localhost:8000/api/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@test.com","password":"test123"}'

# Логин
curl -X POST http://localhost:8000/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"test123"}'
```

### Проверка БД
```bash
docker-compose exec postgres psql -U postgres -d bookmark_manager -c "\dt"
```

## Очистка тестовых данных

```bash
# Сброс БД
docker-compose down -v
docker-compose up -d postgres
docker-compose run --rm backend diesel migration run
```

