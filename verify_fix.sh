#!/bin/bash

echo "========================================="
echo "  Проверка исправления cargo: not found"
echo "========================================="
echo ""

echo "Шаг 1: Очистка старых образов..."
docker-compose down -v 2>/dev/null
docker rmi urlbookmarkmanager-test 2>/dev/null
echo "✓ Очистка завершена"
echo ""

echo "Шаг 2: Сборка нового тестового образа..."
docker-compose -f docker-compose.test.yaml build --no-cache test
echo "✓ Образ собран"
echo ""

echo "Шаг 3: Проверка наличия cargo в образе..."
docker-compose -f docker-compose.test.yaml run --rm test which cargo
if [ $? -eq 0 ]; then
    echo "✓ cargo найден в образе!"
else
    echo "✗ cargo НЕ найден в образе"
    exit 1
fi
echo ""

echo "Шаг 4: Запуск тестов..."
docker-compose -f docker-compose.test.yaml up --abort-on-container-exit
RESULT=$?
echo ""

if [ $RESULT -eq 0 ]; then
    echo "========================================="
    echo "  ✅ ВСЕ ПРОВЕРКИ ПРОЙДЕНЫ!"
    echo "========================================="
    echo ""
    echo "Тесты успешно выполнены."
    echo "Проблема 'cargo: not found' исправлена!"
else
    echo "========================================="
    echo "  ❌ ПРОВЕРКА ПРОВАЛЕНА"
    echo "========================================="
    echo ""
    echo "Проверьте логи выше для деталей."
    exit 1
fi

echo ""
echo "Очистка..."
docker-compose -f docker-compose.test.yaml down
echo "✓ Готово"

