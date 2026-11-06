"""Apply WYR's four visual choices to a staged Horizon checkout.

Only the caller's checkout is changed. Anchors fail closed on incompatible sources.
No resource downloads, game saves, launchers or build commands are touched.
"""
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent / "visual"
RENDER = "crates/horizon-client-render/src/"
CLIENT = "crates/horizon-client/src/"
MARKER = "// WYR visual patch v1"


def apply(root: Path):
    root = Path(root).resolve()
    pending = {}

    def edit(relative, action):
        path = root / relative
        text = path.read_text(encoding="utf-8")
        if MARKER in text:
            return
        updated = action(text)
        pending[path] = updated + "\n" + MARKER + "\n"

    def replace(text, old, new):
        count = text.count(old)
        if count != 1:
            raise RuntimeError(f"Horizon visual anchor count {count}, expected 1: {old[:130]!r}")
        return text.replace(old, new, 1)

    def insert_function(text, name, code):
        pattern = r"((?:pub(?:\([^)]*\))? )?fn " + name + r"\([^{}]*?\)\s*(?:->[^\n{]+)?\s*\{)"
        matches = list(re.finditer(pattern, text, re.S))
        if len(matches) != 1:
            raise RuntimeError(f"Expected one function {name}, found {len(matches)}")
        at = matches[0].end()
        return text[:at] + "\n    " + code + text[at:]

    edit(RENDER + "lib.rs", lambda s: s + "\npub mod wyr_visuals;\n")
    pending[root / (RENDER + "wyr_visuals.rs")] = (HERE / "render_helpers.rs").read_text(encoding="utf-8")
    edit(RENDER + "post_chain.wgsl", lambda s: s + (HERE / "post_effects.wgsl").read_text(encoding="utf-8"))

    def post(s):
        s = replace(s, 'const FRAGMENT: [(&str, &str, &str, usize); 6] = [',
                    'const FRAGMENT: [(&str, &str, &str, usize); 12] = [\n' +
                    '\n'.join(f'    ("wyr:post/{n}", "fs_wyr_{n}", "", 1),' for n in
                              ('invert', 'grayscale', 'scanline', 'posterize', 'chromatic', 'edges')))
        s = replace(s, '''        let Ok(text) = std::fs::read_to_string(resource_path(root, id)) else {
            return Ok(None);
        };
        let config = Config::parse(&text)?;''', '''        let config = if let Some(config) = wyr_post_config(id) { config } else {
            let Ok(text) = std::fs::read_to_string(resource_path(root, id)) else { return Ok(None); };
            Config::parse(&text)?
        };''')
        return s + '''
pub fn wyr_random_effect() -> Option<String> { crate::wyr_visuals::random_effect() }
fn wyr_post_config(id: &str) -> Option<Config> {
    let number: usize = id.strip_prefix("wyr:random_")?.parse().ok()?;
    let name = ["invert", "grayscale", "scanline", "posterize", "chromatic", "edges"].get(number)?;
    let swap = "wyr:random_swap";
    let input = |id: &str| Input { sampler: "In".into(), source: Source::Target { id: id.into(), depth: false }, bilinear: true };
    Some(Config {
        targets: BTreeMap::from([(swap.into(), InternalTarget { width: None, height: None, persistent: false, clear: [0.0; 4] })]),
        passes: vec![
            PassConfig { vertex: "minecraft:core/screenquad".into(), fragment: format!("wyr:post/{name}"), inputs: vec![input(MAIN)], output: swap.into(), uniforms: BTreeMap::new() },
            PassConfig { vertex: "minecraft:core/screenquad".into(), fragment: "minecraft:post/blit".into(), inputs: vec![input(swap)], output: MAIN.into(), uniforms: BTreeMap::from([("BlitConfig".into(), vec![Uniform::Vec4([1.0; 4])])]) },
        ],
    })
}
'''
    edit(RENDER + "post_chain.rs", post)
    edit(CLIENT + "game.rs", lambda s: replace(s,
        '.chain(self.spectated_post_effect.map(str::to_owned))',
        '.chain(self.spectated_post_effect.map(str::to_owned))\n            .chain(horizon_client::post_chain::wyr_random_effect())'))

    def ui(s):
        s = insert_function(s, 'hovered', 'let [x,y,w,h] = crate::wyr_visuals::effective_rect([x,y,w,h]);')
        for name in ('button_as', 'action_button'):
            s = insert_function(s, name, 'let [x,y,w,h] = crate::wyr_visuals::jitter([x,y,w,h]);')
        return s
    edit(RENDER + "ui.rs", ui)
    edit(RENDER + "graphics_style.rs", lambda s: insert_function(s, 'button', 'let rect = crate::wyr_visuals::jitter(rect);'))

    def widgets(s):
        s = insert_function(s, 'end_frame', 'crate::wyr_visuals::end_frame(frames);')
        for name in ('target_as', 'declare_action'):
            s = insert_function(s, name, 'let rect = crate::wyr_visuals::effective_rect(rect);')
        s = insert_function(s, 'record_as', 'let [x,y,w,h] = crate::wyr_visuals::effective_rect([x,y,w,h]);')
        return replace(s, '''    if NARRATING.get() {
        NARRATION.with_borrow_mut(|(current, _)| {
            current.screen = Some(info());''', '''    if NARRATING.get() || horizon_world::wyr_shared::client_count("shaky_buttons") > 0 {
        let info = info();
        crate::wyr_visuals::begin_screen(&info.id);
        if !NARRATING.get() { return; }
        NARRATION.with_borrow_mut(|(current, _)| {
            current.screen = Some(info);''')
    edit(RENDER + "widgets.rs", widgets)
    edit(CLIENT + "mobs/anim.rs", lambda s: replace(s, 'pub fn setup(', 'fn wyr_original_setup(') +
         (HERE / "avatar_freeze.rs").read_text(encoding="utf-8"))

    active = 'horizon_world::wyr_shared::client_count("unsupported_transparency") > 0'
    def gfx(s):
        s = replace(s, 'if self.hdr.is_some() { 1.0 } else { 0.0 },\n                0.0,',
                    'if self.hdr.is_some() { 1.0 } else { 0.0 },\n                if !frame.title && ' + active + ' { 1.0 } else { 0.0 },')
        s = replace(s, 'let oit_on = self.improved_transparency', 'let oit_on = self.improved_transparency\n            && !(' + active + ')')
        s = replace(s, 'let pack_terrain = !frame.title', 'let pack_terrain = !frame.title && !(' + active + ')')
        s = replace(s, '.filter(|_| !frame.title)\n            .and_then(|n| Some((n.material.as_ref()?, &n.frame.group)))',
                    '.filter(|_| !frame.title && !(' + active + '))\n            .and_then(|n| Some((n.material.as_ref()?, &n.frame.group)))')
        return insert_function(s, 'mark_translucent_terrain', 'if ' + active + ' { return; }')
    edit(RENDER + "gfx.rs", gfx)

    def terrain(s):
        for threshold in ('0.5 && (in.layer & 0x8000u) == 0u', '0.01'):
            old = 'if color.a < ' + threshold + ' { discard; }'
            s = replace(s, old, 'if globals.hdr_light.w > 0.5 { color.a = 1.0; } else ' + old)
        return s
    edit(RENDER + "terrain.wgsl", terrain)
    edit(RENDER + "visbuffer.wgsl", lambda s: replace(s,
        'if alpha < 0.5 && (in.layer & 0x8000u) == 0u { discard; }',
        'if globals.hdr_light.w <= 0.5 && alpha < 0.5 && (in.layer & 0x8000u) == 0u { discard; }'))
    # Moving blocks use entity vertices. Reserve overlay 39 only for their geometry;
    # other entities retain cutout/translucent textures. Existing overlays stop at 38.
    edit(CLIENT + "game/moving_pistons.rs", lambda s: replace(replace(s,
        '        self.tinted_block_model_vertices(', '        let wyr_start = out.len();\n        self.tinted_block_model_vertices('),
        '            out,\n        );', '            out,\n        );\n        for vertex in &mut out[wyr_start..] { vertex.layer = (vertex.layer & 0xffff0000) | 39; }'))
    edit(RENDER + "entity.wgsl", lambda s: replace(s,
        '    if color.a < 0.1 || (in.cull != 0u && !front) { discard; }',
        '    if in.overlay == 39u && globals.hdr_light.w > 0.5 { color.a = 1.0; }\n    if color.a < 0.1 || (in.cull != 0u && !front) { discard; }'))

    # All anchors are checked before the first write; an incompatible checkout stays intact.
    for path, content in pending.items():
        with path.open("w", encoding="utf-8", newline="\n") as handle:
            handle.write(content)
    return {"modified": [str(p.relative_to(root)) for p in pending],
            "choices": ["no_animations", "random_shaders", "shaky_buttons", "unsupported_transparency"],
            "api": ["horizon_world::wyr_shared::client_count(&str)->u32", "horizon_world::wyr_shared::entity_count(i32,&str)->u32"]}


if __name__ == "__main__":
    import json
    import sys
    print(json.dumps(apply(Path(sys.argv[1])), indent=2))
