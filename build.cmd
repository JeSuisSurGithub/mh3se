@echo off

del /Q mh3se-release.zip 2>nul
rmdir /S /Q mh3se-release 2>nul
mkdir mh3se-release

cmd /C "set BUILD_INST=cli && cargo build --bin mh3se-cli --release --target x86_64-pc-windows-msvc"
cmd /C "set BUILD_INST=webui && cargo build --bin mh3se-webui --release --target x86_64-pc-windows-msvc"

copy target\x86_64-pc-windows-msvc\release\mh3se-cli.exe mh3se-release\mh3se-cli.exe
copy target\x86_64-pc-windows-msvc\release\mh3se-webui.exe mh3se-release\mh3se-webui.exe

xcopy web mh3se-release\web /E /I /Y
copy LICENSE.md mh3se-release\LICENSE.md
copy README.md mh3se-release\README.md

powershell -Command "Compress-Archive -Path 'mh3se-release\*' -DestinationPath 'mh3se-release.zip' -Force"

rmdir /S /Q mh3se-release