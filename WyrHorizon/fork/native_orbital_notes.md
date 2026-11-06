# Launcher-native Orbital fragment

`native_orbital.py` assembles the real Orbital engine into permitted bundle
files for the single combined `.hnmod`. It does not patch a Horizon checkout or
ship a separate game executable.

## Calling from the combined builder

```python
sys.path.insert(0, str(BASE / 'fork'))
import native_orbital
part = native_orbital.assemble(combined_bundle_directory)
combined_manifest += part['manifest']
```

The caller owns one `[mod]` and one `[compatibility]` header. The fragment has
neither, preventing competing IDs or revision pins. Its revision argument is
checked against `3a817511807aa1056cfcf2598a05180737751cc1`.

The output is:

- `[[crate]] id="orbital_engine", path="orbital_engine"`, package
  `orbital-laser`0.5.10, dependent defaults to `horizon-client`.
- `[[module]] orbital_glue`: `game::orbital_glue` at
  `crates/horizon-client/src/game.rs`.
- `[[module]] orbital_post`: public `orbital_post` at
  `crates/horizon-client-render/src/lib.rs`.
- `[[module]] orbital_carry`: public `orbital_carry` at
  `crates/horizon-client/src/client_lib.rs`.
- Twelve HEAD injections with IDs `orbital_tick`, `orbital_frame`,
  `orbital_net_from_client`, `orbital_net_from_server`,
  `orbital_level_tick_post`, `orbital_camera_shake`, `orbital_world_drawn`,
  `orbital_render_needs`, `orbital_render_frame`, `orbital_carry_tick`,
  `orbital_client_level_load`, `orbital_client_level_unload`.
- No orbital wheel, command, item-key or screen injection.

The existing WYR adapter remains required as `game::wyr::catastrophes`, including
the fixed cached-owner routing/authorization. This generator keeps those glue
calls and the current force-position API. It shares WYR's enabled state and
checks the combined content mod id `wyr`.

`orbital-fragment.toml` is a copy of the returned fragment for inspection. It is
not an independent mod and must not be installed as a separate root manifest.

## Engine and shaders

The generator reuses current `orbital_integration.engine` in a private temp
directory so forced numbers1–6, explicit target coordinates and
`WYR_SMOKE_NO_TERRAIN` stay identical to the tested version.

It compiles and runs upstream `mod/build.rs` DURING PACKAGE GENERATION only.
The bundle contains pregenerated `orbital_engine/gen/passes.rs` and all WGSL
under `orbital_engine/data/{gen,shaders}`. Every include uses a relative path;
no OUT_DIR, absolute local path, build.rs, Cargo.lock or executable is shipped.
The engine has only one registry dependency, `log="0.4"`, already locked by
Horizon. Shader includes in the crystal/rod test source are rewritten too.

The MIT notice is included at `orbital_engine/data/LICENSE-Orbital.txt`.
Orbital content assets/sounds still come from the combined root packager.
The ordinary `.hnmod` content-discovery hook is the combined root builder's
responsibility, since its archive filename belongs to the root.

## Checks completed

- Assembly succeeded:72 generated files,5779087 bytes.
- Two assemblies produced identical SHA256 maps for every emitted file.
- TOML parses as one crate, three modules and twelve injections.
- No build script, Cargo.lock or executable in the generated bundle.
- Standalone copy of the schema-safe crate completed `cargo check`; one
  unchanged upstream unused-import warning.
- All shader includes resolve inside the generated crate.

The combined mixin transform/full game build is performed by the root agent;
this crate check does not establish that every combined native injection can
be transformed by the launcher's pinned mixin version.
