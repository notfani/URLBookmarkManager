Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "  Проверка исправления cargo: not found" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Шаг 1: Очистка старых образов..." -ForegroundColor Yellow
docker-compose down -v 2>$null
docker rmi urlbookmarkmanager-test 2>$null
Write-Host "✓ Очистка завершена" -ForegroundColor Green
Write-Host ""

Write-Host "Шаг 2: Сборка нового тестового образа..." -ForegroundColor Yellow
docker-compose -f docker-compose.test.yaml build --no-cache test
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Ошибка при сборке образа" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Образ собран" -ForegroundColor Green
Write-Host ""

Write-Host "Шаг 3: Проверка наличия cargo в образе..." -ForegroundColor Yellow
docker-compose -f docker-compose.test.yaml run --rm test which cargo
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ cargo найден в образе!" -ForegroundColor Green
} else {
    Write-Host "✗ cargo НЕ найден в образе" -ForegroundColor Red
    exit 1
}
Write-Host ""

Write-Host "Шаг 4: Запуск тестов..." -ForegroundColor Yellow
docker-compose -f docker-compose.test.yaml up --abort-on-container-exit
$RESULT = $LASTEXITCODE
Write-Host ""

if ($RESULT -eq 0) {
    Write-Host "=========================================" -ForegroundColor Green
    Write-Host "  ✅ ВСЕ ПРОВЕРКИ ПРОЙДЕНЫ!" -ForegroundColor Green
    Write-Host "=========================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Тесты успешно выполнены." -ForegroundColor Green
    Write-Host "Проблема 'cargo: not found' исправлена!" -ForegroundColor Green
} else {
    Write-Host "=========================================" -ForegroundColor Red
    Write-Host "  ❌ ПРОВЕРКА ПРОВАЛЕНА" -ForegroundColor Red
    Write-Host "=========================================" -ForegroundColor Red
    Write-Host ""
    Write-Host "Проверьте логи выше для деталей." -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Очистка..." -ForegroundColor Yellow
docker-compose -f docker-compose.test.yaml down
Write-Host "✓ Готово" -ForegroundColor Green

