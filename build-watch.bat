@echo off
REM Watch mode - automatically rebuild on file changes
call "%~dp0build.bat" --watch
