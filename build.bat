@echo off
REM Wiki Builder
cd /d "%~dp0builder"
if "%~1"=="" (
    cargo run -- --build
) else (
    cargo run -- %*
)
