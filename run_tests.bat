@echo off
setlocal enabledelayedexpansion

echo ========================================
echo   Bookmark Manager - Test Runner
echo ========================================
echo.

if "%1"=="--standalone" (
    echo Mode: Standalone ^(separate test environment^)
    echo.
    echo Starting test environment...
    docker-compose -f docker-compose.test.yaml up --build --abort-on-container-exit

    echo.
    echo Cleaning up...
    docker-compose -f docker-compose.test.yaml down
) else (
    echo Mode: With full stack
    echo.
    echo Step 1: Starting all services...
    docker-compose up -d postgres backend frontend nginx

    echo.
    echo Step 2: Waiting for services to be ready...
    timeout /t 10 /nobreak >nul

    echo.
    echo Step 3: Running tests...
    docker-compose up --build test

    echo.
    echo Step 4: Viewing test results...
    docker-compose logs test
)

echo.
echo ========================================
echo   Tests completed!
echo ========================================
echo.
echo Usage tips:
echo   Full stack with tests:  run_tests.bat
echo   Only tests ^(isolated^):  run_tests.bat --standalone
echo.
pause

