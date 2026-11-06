# Launcher-native gameplay and visual fragments

`native_patches.assemble(destination, source_root=None)` returns:

- `manifest`: TOML fragment containing `[[module]]` and `[[injection]]`, without a second `[mod]`.
- `content`: mapping of resource paths to bytes (13 owned GLSL/post-effect JSON assets).
- `files`: generated helper/module filenames, plus diagnostic filenames.
- `gaps`: explicit remaining adaptations.

The same assets are emitted under `destination/content-assets/` for inspection; put them at the **content root** of the final `.hnmod`, not under its mixins directory. `emit` aliases `assemble`. Default source is the pinned local Horizon checkout. The generator never edits that checkout, launcher configuration, a live world, or a compiled stage. It reads the existing gameplay/visual patchers using a disposable sparse temporary copy to derive matching source bodies.

Use the root `fork/build_native.py` to merge these fragments with WYR, Upgrader and Orbital. Current verified combination: Horizon `3a817511807aa1056cfcf2598a05180737751cc1`, launcher mixins `a2af5c0dd4117d51ecd70066ba7a00b4a2c10dab`. The combined native plan passed with **99 injections, 12 modules, 1 vendored crate, zero post-build commands**. Evidence: `native-launcher/plan-full.json` and `native-launcher/plan-fragments.log`. Planning is not a Rust compile or game test.

## Implementation

Function bodies with closures, `?`, macros or owned early returns are preserved by cancellable HEAD. The helper executes the adapted body in an ordinary Rust closure and returns `ControlFlow::Break`. This uses the supported boundary contract rather than requesting unsupported OVERWRITE traversal. Binding expressions are only explicit parameters/borrows; owned input cloning happens inside helper code so the original Continue branch remains type-correct. These canceling injections use `zz_native_` IDs to run after other same-priority HEAD hooks, notably Orbital's Renderer frame snapshot.

The renderer's non-Clone `GpuChecks` never gets cloned or moved into a HEAD binding: an added inherent `Renderer::wyr_native_draw_frame` owns the adapted body, while cancellable wrappers of the ordinary `render` and `draw_scaled_frame` entry points route calls to it. This preserves the original native callbacks in the drawing body.

Terrain WGSL is substituted at synchronous `gfx::reuse::On::create_shader_module`, keeping the real cache key and object reuse. Source line endings are normalized before matching because Windows `include_str!` embeds CRLF while Python reads LF. Entity WGSL is supplied by a concrete String-returning `gfx::entity_shader` HEAD. The existing `hdr_light.w` uniform carries the active transparency flag; OIT/custom terrain material paths are disabled while selected. GPU visibility-buffer drawing is bypassed while the effect is active, so an unmodified visibility cutout shader cannot hide the newly opaque texels.

The adapted random effects use actual GLSL resources and stock Horizon `post_glsl::translate`, avoiding changes to `PostEffects::new` or module constants. There are six effects, ten-second stable random selection, no immediate repeats, and removal outside an active client. Each uses an intermediate target and a final blit. All six vertex/fragment pairs passed the **exact upstream translator and its Naga validation** in an isolated test executable; source in `native-fragments-check/validate_post.rs`, executable `validate_post.exe`. The owning game compilation and interactive native-package smoke test are still required.

Gameplay includes enderman riding seat/dismount, frog player-food goal, cobweb suppression, shovel-mineable noclip, faster pig riding, spider climbing suppression, sinking boats and creeper flower replacement. The source fork's generic `as_guest_with_host<R>` actor scope is replaced with a push/pop stack around concrete `enter_guest` / `leave_guest` hooks. Public `horizon_world::wyr_shared::ActorGuard` and count APIs must be present in the WYR shared module.

NO_ANIMATIONS retains player poses through a helper module. Standard sprite and action buttons shake with persistent random-walk positions and matching shared hover/controller rectangles. Generic custom narration callbacks are dropped normally when the standard button wrapper cancels; the replacement records normal button narration.

## Remaining adaptations

`graphics_style::button` has a destructured tuple parameter. The mixins index rejects that signature before inspecting helper bindings (`index::parameters`), so manually listing the tuple's names cannot make it a legal target. The fragment covers standard sprite/action buttons; flat-style separately drawn custom controls remain unshifted. `widgets::begin_screen` owns an opaque `FnOnce` callback: consuming it twice would be incorrect, so its screen-ID reset is omitted; caches still expire unused rectangles and clear when the choice is inactive. These gaps are not represented as implemented hooks.

Helpers that replace whole original bodies intentionally rely on this exact source revision and may conflict with another mod's interior rewrites of those functions. Same-priority HEAD callbacks run first by the explicit ID ordering, but arbitrary third-party interior hooks cannot be preserved merely by planning both mods successfully. Final delivery must retain the source pin.
