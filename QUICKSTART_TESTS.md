# 🚀 Быстрый старт - Автоматические тесты

## ✨ Что изменилось?

Теперь при запуске проекта **автоматически выполняются все тесты**!

## 📋 Способы запуска

### 1️⃣ Полный запуск (проект + тесты)

```bash
docker-compose up --build
```

**Что происходит:**
1. ✅ Запускается PostgreSQL
2. ✅ Запускается Backend API
3. ✅ Запускается Frontend
4. ✅ Запускается Nginx
5. 🧪 **Автоматически запускаются все 40+ тестов**
6. 📊 Результаты выводятся в консоль

**Посмотреть результаты:**
```bash
docker-compose logs test
```

---

### 2️⃣ Только тесты (без остальных сервисов)

**PowerShell:**
```powershell
.\run_tests.ps1 -Standalone
```

**CMD:**
```cmd
run_tests.bat --standalone
```

**Или напрямую:**
```bash
docker-compose -f docker-compose.test.yaml up --build
```

**Что происходит:**
1. ✅ Запускается только PostgreSQL
2. 🧪 Запускаются все тесты
3. 🧹 Окружение автоматически очищается

---

### 3️⃣ Тесты в уже работающем проекте

Если проект уже запущен:

```bash
docker-compose up test
docker-compose logs test
```

---

## 📊 Интерпретация результатов

### ✅ Успех
```
========================================
  ✅ All tests passed successfully!
========================================
test result: ok. 40 passed; 0 failed
```

### ❌ Ошибка
```
test result: FAILED. 35 passed; 5 failed
```

**См. подробности:** [TEST_RESULTS.md](TEST_RESULTS.md)

---

## 🎯 Структура тестов

- **👤 User Tests (5)** - регистрация, аутентификация
- **📁 Category Tests (5)** - создание, получение категорий
- **🔖 Bookmark Tests (10)** - CRUD операции, поиск
- **🔐 Auth Tests (4)** - JWT токены
- **🌐 Handler Tests (12)** - HTTP API endpoints
- **🔄 E2E Tests (3)** - полные сценарии

**Всего: 40+ тестов**

---

## ⚙️ Конфигурация

### Docker образы

#### Production vs Testing

**backend/Dockerfile** (Production):
- Многоступенчатая сборка
- Финальный образ: `debian:bookworm-slim`
- Размер: ~100MB
- Содержит: только скомпилированный бинарник
- Быстрый запуск, минимальные ресурсы

**backend/Dockerfile.test** (Testing):
- Одноступенчатая сборка
- Базовый образ: `rust:1.86`
- Размер: ~1.5GB
- Содержит: cargo, rustc, diesel_cli, исходный код
- Позволяет запускать `cargo test`

### docker-compose.yaml
Основной файл с сервисом `test`:
- Запускается автоматически при `docker-compose up`
- Зависит от `postgres` (healthcheck)
- Выполняет миграции
- Запускает все тесты
- `restart: "no"` - выполняется один раз

### docker-compose.test.yaml
Изолированный файл только для тестов:
- Только PostgreSQL + тесты
- Отдельная сеть
- Автоматическая очистка

---

## 🔧 Отладка

### Тесты не запустились?

```bash
# Проверьте статус контейнеров
docker-compose ps

# Проверьте логи PostgreSQL
docker-compose logs postgres

# Проверьте логи тестов
docker-compose logs test

# Перезапустите окружение
docker-compose down -v
docker-compose up --build
```

### Хотите подробный вывод?

```bash
# Отредактируйте docker-compose.yaml, добавьте флаги:
cargo test -- --nocapture --test-threads=1
```

---

## 📚 Дополнительные ресурсы

- **[TESTING.md](TESTING.md)** - полная документация по тестам
- **[TEST_RESULTS.md](TEST_RESULTS.md)** - как читать результаты
- **[COMMANDS.md](COMMANDS.md)** - все команды проекта
- **[README.md](README.md)** - основная документация

---

## 🎓 Для CI/CD

### GitHub Actions

```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run tests
        run: docker-compose -f docker-compose.test.yaml up --abort-on-container-exit
```

### GitLab CI

```yaml
test:
  stage: test
  script:
    - docker-compose -f docker-compose.test.yaml up --abort-on-container-exit
```

---

## 💡 Советы

1. **Первый запуск будет долгим** (скачивание образов + компиляция)
2. **Последующие запуски быстрее** (~15 секунд для тестов)
3. **Используйте `--build`** после изменений в коде
4. **Логи сохраняются** - можно просмотреть позже
5. **Тесты изолированы** - каждый создаёт свои данные

---

## ✅ Проверочный список

- [ ] Docker Desktop запущен
- [ ] Проект склонирован
- [ ] Выполнена команда `docker-compose up --build`
- [ ] Дождались завершения тестов
- [ ] Проверили логи: `docker-compose logs test`
- [ ] Увидели: "All tests passed successfully!"

**Готово! 🎉**

