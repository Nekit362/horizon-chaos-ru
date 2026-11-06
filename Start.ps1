$ErrorActionPreference='Stop'
$bundleRoot=$PSScriptRoot
$gameRoot=Join-Path $bundleRoot 'game'
$binary=Join-Path $bundleRoot 'horizon-client.exe'
if(!(Test-Path -LiteralPath $binary)){throw 'Рядом со Start.ps1 должен быть horizon-client.exe из Windows-пакета Release.'}
$versions=Join-Path $env:APPDATA 'horizon-launcher/versions'
$resources=Get-ChildItem -LiteralPath $versions -Directory -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending | ForEach-Object {Join-Path $_.FullName '.local/horizon-client/26.3'} | Where-Object {Test-Path -LiteralPath $_} | Select-Object -First 1
if(!$resources){throw 'Откройте Horizon Launcher и подготовьте Minecraft Java 26.3. Подготовленные ресурсы не найдены.'}
New-Item -ItemType Directory -Force -Path $gameRoot,(Join-Path $gameRoot 'horizon-mods') | Out-Null
$optionsFile=Join-Path $gameRoot 'options.txt'
if(!(Test-Path -LiteralPath $optionsFile)){Set-Content -LiteralPath $optionsFile -Value 'lang:ru_ru' -Encoding ASCII}
$content=Join-Path $bundleRoot 'wyr-full-content-0.2.0.hmod'
if(Test-Path -LiteralPath $content){Copy-Item -LiteralPath $content -Destination (Join-Path $gameRoot 'horizon-mods/wyr-full.hmod') -Force}
$env:HORIZON_HOME=Join-Path $gameRoot 'horizon-home'
$env:HORIZON_LAUNCHER_HOME=Join-Path $gameRoot 'launcher-home'
$env:XDG_DATA_HOME=Join-Path $gameRoot 'xdg-data'
$env:XDG_CONFIG_HOME=Join-Path $gameRoot 'xdg-config'
$env:HORIZON_LAUNCHER_NO_SELF_UPDATE='1'
Push-Location $gameRoot
try {& $binary --resources $resources --game-dir $gameRoot --home $env:HORIZON_HOME --player-dir $gameRoot --dev-mod (Join-Path $gameRoot 'horizon-mods/wyr-full.hmod')} finally {Pop-Location}
