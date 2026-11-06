# Isolated Windows game test setup

Read-only discovery completed 2026-11-06. No installed player data, accounts,
worlds, launcher settings or resources were modified.

## Existing local resources and Java

Prepared Minecraft26.3 resources are now present and complete in both:

```text
C:\Users\nikit\AppData\Roaming\horizon-launcher\versions\nightly-2026-11-05-66f3981\.local\horizon-client\26.3
C:\Users\nikit\AppData\Roaming\horizon-launcher\versions\native-e57b1d3-0635302fdd9e\.local\horizon-client\26.3
```

Each matching installed `horizon-prepare.exe --check --root VERSION_FOLDER`
returned exit0. The nightly prepared tree contains28180 files,741909240 bytes.
These checks establish completeness for the installed preparer; a smoke launch
must still confirm compatibility with our pinned engine.

The bundled runtime is a full JDK, including `bin\javac.exe`:

```text
C:\Users\nikit\AppData\Roaming\horizon-launcher\minecraft\runtime\java-runtime-epsilon\windows-x64\java-runtime-epsilon
```

Running its `bin\java.exe -version` reports Microsoft OpenJDK25.0.1.
Existing source Minecraft files are available under the launcher's
`minecraft\versions\26.3`, `minecraft\assets` and `minecraft\libraries`.
Server jar:

```text
C:\Users\nikit\AppData\Roaming\horizon-launcher\server-jars\26.3.jar
```

## Launch flags confirmed

The installed nightly `horizon-client.exe --launch-api` returned API0.1.0,
Minecraft26.3 and supports `--resources`, `--game-dir`, `--home`,
`--player-dir`, `--width`, `--height`. The pinned source additionally supports
`--world NAME` (creates a new scratch world), `--creative`, `--test-terrain`,
`--control`, `--control-socket PATH`, `--no-pause`, `--world-frames N`,
`--timeout-seconds N`, `--smoke-report PATH` and `--screenshot-after N`.
`--headless` does not create a renderer and cannot verify Orbital GPU effects.

Use a private copied resource tree so even unexpected cache writes cannot
touch the launcher's installed resources. Example PowerShell setup, to be run
by the root agent once the custom binary exists:

```powershell
$testRoot = 'C:\Users\nikit\Documents\WyrHorizon\playtest'
$resourceSource = 'C:\Users\nikit\AppData\Roaming\horizon-launcher\versions\nightly-2026-11-05-66f3981\.local\horizon-client\26.3'
New-Item -ItemType Directory -Force -Path "$testRoot\resources", "$testRoot\game", "$testRoot\home", "$testRoot\launcher", "$testRoot\xdg-data", "$testRoot\xdg-config" | Out-Null
Get-ChildItem -LiteralPath $resourceSource -Force | Copy-Item -Destination "$testRoot\resources" -Recurse
$env:HORIZON_HOME = "$testRoot\home"
$env:HORIZON_LAUNCHER_HOME = "$testRoot\launcher"
$env:XDG_DATA_HOME = "$testRoot\xdg-data"
$env:XDG_CONFIG_HOME = "$testRoot\xdg-config"
$env:XDG_RUNTIME_DIR = "$testRoot\runtime"
$env:HORIZON_LAUNCHER_NO_SELF_UPDATE = '1'
$env:HORIZON_NO_AUDIO = '1'
$binary = 'C:\Users\nikit\Documents\WyrHorizon\target-shared\debug\horizon-client.exe'
& $binary --resources "$testRoot\resources" --game-dir "$testRoot\game" --home "$testRoot\home" --player-dir "$testRoot\game" --world WYR-SCRATCH --creative --width 1280 --height 720 --render-distance 4 --control --control-socket "$testRoot\runtime\control.sock"
```

Install only the combined mod's content in `playtest\game\horizon-mods`.
Do not copy installed saves, account-session files, options or credentials.
Use the owned scratch world. Orbital's AGENTS.md requires a first strike with
terrain disabled and user confirmation before the first terrain-changing run.
The root agent should first run the nondestructive visual test below.

For unattended startup verification add `--world-frames 120 --timeout-seconds 180
--smoke-report ABSOLUTE_JSON_PATH`. For visual tests keep the real renderer
enabled. If using `Start-Process`, use `-WindowStyle Hidden` for helper processes;
use a visible game only for a UI session the user needs to observe or control.

## Exact Orbital terrain-off setting

The native engine uses a plain-text storage file, not JSON:

```text
C:\Users\nikit\Documents\WyrHorizon\playtest\game\horizon-mods\.horizon-mod-data\orbital\settings
```

Its exact line is `terrain=0` (newline terminated); `terrain=1` enables edits.
This is loaded by `Orbital::load` / `Orbital::parse` in `mod/src/lib.rs`.
The native glue gets this path from `game.native_data_dir("orbital")`.

IMPORTANT: the combined forced WYR API currently sets `m.terrain=true` just
before calling `fire`, so seeding this settings file alone does not disable
forced-event terrain. Before testing a forced catastrophe, root must add a
test-only override to `fork/orbital_integration.py`, for example:

```rust
m.terrain = std::env::var_os("WYR_SMOKE_NO_TERRAIN").as_deref()
    != Some(std::ffi::OsStr::new("1"));
```

Then rebuild and set `$env:WYR_SMOKE_NO_TERRAIN='1'` for the scratch process.
Without that change, testing only startup/UI is nondestructive; do not trigger
the forced catastrophe based on `terrain=0` alone.

## If the copied resource version is incompatible

Prepare from the already installed user's Minecraft into an owned stage and
scratch output. No downloads or credentials are required for this path.

```powershell
$prepareRoot = 'C:\Users\nikit\Documents\WyrHorizon\modded-resumed'
$testRoot = 'C:\Users\nikit\Documents\WyrHorizon\playtest'
$jdk25 = 'C:\Users\nikit\AppData\Roaming\horizon-launcher\minecraft\runtime\java-runtime-epsilon\windows-x64\java-runtime-epsilon'
$env:JAVA_HOME = $jdk25
$env:PATH = "$jdk25\bin;$env:PATH"
& 'C:\Users\nikit\AppData\Roaming\horizon-launcher\versions\nightly-2026-11-05-66f3981\horizon-prepare.exe' --prepare --complete --root $prepareRoot --output "$testRoot\resources-reprepared" --minecraft-dir 'C:\Users\nikit\AppData\Roaming\horizon-launcher\minecraft' --server-jar 'C:\Users\nikit\AppData\Roaming\horizon-launcher\server-jars\26.3.jar' --java-home $jdk25
```

The installed preparer may expect its matching exporter contract. If it rejects
the pinned stage, build/use that stage's own `horizon-prepare` with the same
arguments rather than changing installed exporter files. The preparer writes
scratch imports/exporter outputs only under the explicit owned root/output.

No game test has been launched by this discovery task.
