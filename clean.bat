@echo off
echo ========================================
echo    Roblox Booster - Clean Script
echo ========================================
echo.

echo Cleaning build artifacts...
cargo clean

if %errorlevel% equ 0 (
    echo.
    echo Successfully cleaned!
    echo All build artifacts have been removed.
) else (
    echo.
    echo ERROR: Clean failed!
)

echo.
pause