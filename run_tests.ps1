param(
    [switch]$Standalone = $false
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Bookmark Manager - Test Runner" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if ($Standalone) {
    Write-Host "Mode: Standalone (separate test environment)" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Starting test environment..." -ForegroundColor Green
    docker-compose -f docker-compose.test.yaml up --build --abort-on-container-exit

    Write-Host ""
    Write-Host "Cleaning up..." -ForegroundColor Yellow
    docker-compose -f docker-compose.test.yaml down
} else {
    Write-Host "Mode: With full stack" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Step 1: Starting all services..." -ForegroundColor Green
    docker-compose up -d postgres backend frontend nginx

    Write-Host ""
    Write-Host "Step 2: Waiting for services to be ready..." -ForegroundColor Green
    Start-Sleep -Seconds 10

    Write-Host ""
    Write-Host "Step 3: Running tests..." -ForegroundColor Green
    docker-compose up --build test

    Write-Host ""
    Write-Host "Step 4: Viewing test results..." -ForegroundColor Green
    docker-compose logs test
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Tests completed!" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Usage tips:" -ForegroundColor Yellow
Write-Host "  Full stack with tests:  .\run_tests.ps1" -ForegroundColor White
Write-Host "  Only tests (isolated):  .\run_tests.ps1 -Standalone" -ForegroundColor White
Write-Host ""

