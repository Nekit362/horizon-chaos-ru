# WYR visuals in the separate Horizon build

Integration: import `fork/visual_patches.py` and call `apply(Path(staged_checkout))` after native-mod staging and gameplay patches, before compiling. The checkout must match Horizon `3a817511807aa1056cfcf2598a05180737751cc1`. The patcher checks every anchor before writing, changes 14 source files, and is idempotent. When changing the patcher itself, regenerate the checkout from the preceding stage rather than applying a newer revision over previously marked files.

The engine must expose these thread-safe public functions in `horizon_world::wyr_shared`:

```rust
pub fn client_count(id: &str) -> u32;
pub fn entity_count(entity_id: i32, id: &str) -> u32;
```

`client_count` must return zero when there is no local player or active connection, and after leaving a world. `entity_count` reports the selected count for a remote player in the current world, and must clear disconnected/world-old IDs. No further callbacks or renderer hooks need registration: the patcher inserts the calls directly. Choice IDs: `no_animations`, `random_shaders`, `shaky_buttons`, `unsupported_transparency`.

## Implemented behavior

- **NO_ANIMATIONS:** local player (render-model ID 0) uses `client_count`; other player models use `entity_count`. Retains each player's last bone poses and suppresses subsequent animation/emote setup while selected. Model-part visibility and spectator visibility still update. Other mobs, the camera and first-person item swing are unaffected, matching the original PlayerEntityModel hook's scope.
- **RANDOM_SHADERS:** six real built-in WGSL effects: invert, grayscale, alternating scanlines, posterization, red/blue channel offset, and edge detection. Random choice changes every ten seconds while active; successive choices differ. The effect is applied as a genuine two-pass post chain with an intermediate target and final blit, and needs no Java shader assets. Removing the choice removes the requested chain. This is an adapted set, not byte-for-byte ports of Java GLSL; timing uses wall time rather than the original 200 client ticks.
- **SHAKY_BUTTONS:** persistent random walk in GUI coordinates, each axis adds an integer in `[-choice_count, choice_count]` per drawn frame. The native sprite/action buttons and flat-style buttons move with their text; shared hover/click and controller registration follow the moved rectangle. Off-screen wandering is intentionally possible as in the source. Original positions return when disabled. Per-screen caches are discarded on screen changes. Defensive caps are 1000 units per frame and one million units of cumulative drift. This adaptation covers shared button drawing, not every separately implemented custom control or slider in Horizon.
- **UNSUPPORTED_TRANSPARENCY:** terrain cutout fragments keep all texels; translucent block/fluid fragments force alpha 1. Visibility-buffer cutout follows the same rule. Moving/transient block-model vertices receive reserved overlay 39, causing only those entity-pipeline vertices to become opaque. Player skins, mob textures and particles retain their normal transparency. OIT, custom terrain-pack pipelines and native custom terrain materials are bypassed while active, avoiding an alternate pipeline silently overriding the effect. The existing uniform's previously unused `hdr_light.w` carries the flag, so the shared GPU uniform layout stays unchanged. Choice loss restores the normal paths without remeshing.

## Verification performed

- `py -3.9 fork/visual/probe.py`: passed all source anchors on isolated copies of the 13 existing input files plus the new helper module; applying twice produced identical bytes.
- `fork/visual/check.bat`: passed both Rust helper tests (random-walk frame bounds, identical drawn/hit rectangles, single movement per frame, disabling reset, stable shader within an interval).
- Naga 30.0.1 parsed and validated the patched complete post-chain shader (15 entry points), terrain (28), entity with actual generated overlay constants (15), and combined terrain/visibility-buffer shader (34).
- No whole-game Cargo build or interactive game session was run by this subtask. Parent integration must compile the final stage and smoke-test the four choices in the game before claiming visual runtime validation. In particular, custom screens and drag gestures warrant interactive testing.

The local validation harness uses the already built Naga rlib under `UpgraderHorizon/target-tools`; it does not modify shared build targets. Files under `fork/visual/probe_source` are disposable test copies, not engine deliverables.
