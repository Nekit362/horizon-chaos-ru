@echo off
call "E:\Visual studio\Main\VC\Auxiliary\Build\vcvars64.bat" >nul
set RUSTUP_HOME=C:\Users\nikit\Documents\WyrHorizon\toolchain\rustup
set CARGO_HOME=C:\Users\nikit\Documents\WyrHorizon\toolchain\cargo
set DEPS=C:\Users\nikit\Documents\UpgraderHorizon\target-tools\debug\deps
cd /d C:\Users\nikit\Documents\WyrHorizon\fork\visual
"%CARGO_HOME%\bin\rustc.exe" --edition 2024 --test helper_tests.rs -o helper_tests.exe
if errorlevel 1 exit /b 1
helper_tests.exe
if errorlevel 1 exit /b 1
"%CARGO_HOME%\bin\rustc.exe" --edition 2024 validate_wgsl.rs --extern naga=%DEPS%\libnaga-f58e70ca09609274.rlib -L dependency=%DEPS% -o validate_wgsl.exe
if errorlevel 1 exit /b 1
validate_wgsl.exe probe_source/crates/horizon-client-render/src/post_chain.wgsl probe_source/crates/horizon-client-render/src/terrain.wgsl probe_source/crates/horizon-client-render/src/entity.wgsl probe_source/crates/horizon-client-render/src/visbuffer.wgsl
