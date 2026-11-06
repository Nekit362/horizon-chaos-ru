# One combined personal package

Run from any working directory:

```powershell
py -3.9 C:/Users/nikit/Documents/WyrHorizon/fork/combined_package.py
```

Creates `dist/combined/wyr-full-content-0.2.0.hmod`, one root manifest with `id = "wyr"`, Russian name/description, 310 entries. WYR, Upgrader and Orbital retain their original resource namespaces. Orbital MIT LICENSE and NOTICE are included. This is content for the separate patched Horizon build: it is **not** a standalone native mod or an `.hnmod`, and does not supply the gameplay implementation to ordinary Horizon.

The local personal content package contains original WYR/Upgrader assets from the user's mods. No GitHub publication was performed. The generated source delivery excludes those protected originals, all JARs, decompiled reference, the entire upstream Horizon engine, toolchains, targets, logs and credential/config files. It retains port-owned Rust, JSON definitions/translations, native helpers, generated bundles, builders, visual/gameplay/Orbital patchers and the required MIT Orbital source/shim/helpers/WGSL/assets/synthesized sounds with LICENSE/NOTICE. The vendored tomli parser includes its MIT license.

Source delivery layout:

```text
bootstrap.py                    pinned upstream retrieval; optional fork build
PROVENANCE.json                 exact Horizon, horizon-mixins, Orbital commits
SOURCE-README-RU.md             Russian rebuild instructions and limitations
WyrHorizon/port/{src,mixins,...} WYR source and native staging bundle
WyrHorizon/port/choice-definitions.json
WyrHorizon/fork/                combined package builder and fork patchers
WyrHorizon/orbital-laser/       only required licensed Orbital subtree
UpgraderHorizon/{src,mixins,tools,...}
```

Generate a new reviewable source folder plus deterministic source ZIP:

```powershell
py -3.9 C:/Users/nikit/Documents/WyrHorizon/fork/combined_package.py --source-dir C:/Users/nikit/Documents/WyrHorizon/dist/combined/source-ready --source-zip C:/Users/nikit/Documents/WyrHorizon/dist/combined/wyr-full-source-0.2.0.zip
```

The source directory must be absent or empty; existing files are never recursively removed. Use a fresh folder when source changes. Run the builder again after other agents finish to include their latest files. The ZIP uses sorted paths, fixed timestamps/Unix permissions and fixed compression settings.

From the delivered source folder, `py -3.9 bootstrap.py` fetches pinned Horizon and horizon-mixins. `py -3.9 bootstrap.py --build --stage full-engine` also creates a fresh native stage from both included bundles (Upgrader then WYR), runs all fork patches, and builds release `horizon-client`. Git, Cargo/Rust and the platform's C/C++ toolchain are external requirements. It does not run the game. Port generation reads `choice-definitions.json`, not decompiled Java.

Recreate the personal `.hmod` from the clean source delivery and supplied original JARs:

```powershell
py -3.9 WyrHorizon/fork/combined_package.py --wyr-jar PATH/wyr-1.0.0_1.jar --upgrader-jar PATH/upgrader-forge-1.3.0-26.1.2.jar --output wyr-full-content-0.2.0.hmod
```

Validation performed: every JSON asset parses; duplicate paths across input packages, including case-only collisions, fail before ZIP creation; archive layout has exactly one `horizon-mod.toml`. Rebuilding from the generated source folder and the actual original JARs produced exactly the same SHA256 as the local-content build: `39ee8ec9cd2a198baf38887fd41f6a1fa9a55b8a558ad07fdb5f7cf5910e2238`. Pure helper collision rejection and byte determinism are also checked. This packaging subtask did not perform an interactive game test or invoke the full engine compiler; those are separate parent validation steps.
