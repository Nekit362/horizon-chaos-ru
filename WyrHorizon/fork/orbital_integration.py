"""Embed upstream Orbital Laser's real native engine into an owned Horizon stage.

Never edits the original Horizon or Orbital source trees. Call apply(stage).
Requires tomli on Python <3.11 (bundled python-deps is accepted).
"""
from pathlib import Path
import json, re, shutil, sys
try:
    import tomllib
except ImportError:
    sys.path.insert(0, str(Path(__file__).parent/'python-deps'))
    import tomli as tomllib

BASE = Path(__file__).resolve().parent.parent
SOURCE = BASE/'orbital-laser'

def replace_once(text, old, new):
    if text.count(old) != 1:
        raise RuntimeError('Orbital integration expected one occurrence: '+old[:110])
    return text.replace(old, new, 1)

def write(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding='utf8')

def engine(stage):
    dst=stage/'crates/wyr-orbital-engine'
    shutil.copytree(SOURCE/'mod/src', dst/'src',dirs_exist_ok=True)
    shutil.copytree(SOURCE/'mod/shaders', dst/'shaders',dirs_exist_ok=True)
    shutil.copy2(SOURCE/'mod/build.rs', dst/'build.rs')
    shutil.copy2(SOURCE/'LICENSE', dst/'LICENSE')
    shutil.copy2(SOURCE/'native/engine-shim/horizon_sdk.rs', dst/'src/horizon_sdk.rs')
    lib=(dst/'src/lib.rs').read_text(encoding='utf8')
    lines=lib.splitlines(); i=0
    while i<len(lines) and (lines[i].startswith('//!') or not lines[i].strip()): i+=1
    lines[i:i]=['#![allow(unused_mut, dead_code, hidden_glob_reexports)]', 'extern crate self as horizon_mod_sdk;', 'pub mod horizon_sdk;', 'pub use horizon_sdk::*;']
    lib='\n'.join(lines)+'\n'
    # Reuse the entire original fire/playback implementation with a forced hit;
    # this bypasses item holding, wheel choice and crosshair raycasting.
    lib+='''
pub fn wyr_force(number:u8, at:[f64;3])->bool {
    let keys=["blackhole","meteor","planetkiller","rod","sinkhole","payload"];
    let Some(key)=number.checked_sub(1).and_then(|n|keys.get(n as usize)) else {return false;};
    if at.iter().any(|v|!v.is_finite()){return false;}
    let mut fired=false;
    __mod_with(|m| {
        if m.strike.is_some(){return;}
        let Some(index)=LASERS.iter().position(|l|l.key==*key) else{return;};
        m.laser=index; m.terrain=std::env::var_os("WYR_SMOKE_NO_TERRAIN").as_deref()!=Some(std::ffi::OsStr::new("1")); m.follow=false; m.skip=0.0;
        horizon_sdk::__host::with(|s|s.snapshot.view_hit=Some((0.0,horizon_sdk::world::Hit {
            block_x:at[0].floor() as i32, block_y:at[1].floor() as i32, block_z:at[2].floor() as i32,
            x:at[0],y:at[1],z:at[2],block:"minecraft:air".into()
        })));
        m.fire(); fired=m.strike.is_some();
    });
    fired
}
'''
    write(dst/'src/lib.rs',lib)
    write(dst/'Cargo.toml','''[package]
name = "orbital-laser"
version = "0.5.10"
edition = "2024"
license = "MIT"
[dependencies]
log = "0.4"
''')
    for name in ('horizon-client','horizon-client-render'):
        path=stage/f'crates/{name}/Cargo.toml'
        text=path.read_text(encoding='utf8')
        if 'orbital-laser = { path = "../wyr-orbital-engine" }' not in text:
            write(path,replace_once(text,'[dependencies]','[dependencies]\norbital-laser = { path = "../wyr-orbital-engine" }'))

def post_source():
    src=(SOURCE/'native/engine-shim/mod_post.rs').read_text(encoding='utf8')
    wgsl=(SOURCE/'native/engine-shim/mod_post.wgsl').read_text(encoding='utf8')
    src=replace_once(src,'include_str!("mod_post.wgsl")','r###"'+wgsl+'"###')
    src=src[:src.index('#[cfg(test)]\nmod tests {')].rstrip()+'\n'
    src=replace_once(src,'        mut frame: FrameUniform,\n    ) -> &wgpu::TextureView {','        mut frame: FrameUniform,\n        target: &wgpu::TextureView,\n    ) -> &wgpu::TextureView {')
    for old in ('draw(encoder, &[Some(&output.view)], pipeline, &group);','draw(encoder, &[Some(&output.view)], join, &join_group);'):
        src=replace_once(src,old,old.replace('&[Some(&output.view)]','&[Some(target)]'))
    return re.sub(r'(?m)^//!','//',src)+'\n'+(SOURCE/'native/engine-shim/post_hooks.rs').read_text(encoding='utf8')

def append_module(path,name,source,public=False):
    old=path.read_text(encoding='utf8')
    marker='mod '+name+' {'
    if marker in old: return
    source=re.sub(r'(?m)^//!','//',source)
    write(path,old+'\n'+('pub ' if public else 'pub(crate) ')+marker+'\n'+source+'\n}\n')

def inject(stage,spec,hooks):
    # Content is merged into the one WYR content package by the parent builder.
    if spec['id']=='content_find': return
    if 'hook' in spec:
        target=hooks[spec['hook']]['target']
    else: target=spec['target']
    fn=target.rsplit('::',1)[1]
    if target.startswith('horizon_client::game::'):
        file=stage/'crates/horizon-client/src/game/native_hooks.rs'
    elif '::native_render::' in target:
        file=stage/'crates/horizon-client-render/src/gfx/native_render.rs'
    elif target.endswith('Renderer::render'):
        file=stage/'crates/horizon-client-render/src/gfx.rs'
    elif 'MotionWorld' in target:
        file=stage/'crates/horizon-client/src/motion.rs'
    else: raise RuntimeError('Unhandled Orbital injection target '+target)
    source=file.read_text(encoding='utf8')
    if '// WYR embedded Orbital: '+spec['id']+'\n' in source: return
    matches=list(re.finditer(r'\bfn\s+'+re.escape(fn)+r'\s*(?:<[^\n]*>)?\s*\(',source))
    if len(matches)!=1: raise RuntimeError(f'{file}: function {fn}: {len(matches)} matches')
    start=source.find('{',matches[0].end())+1
    helper=(SOURCE/'native/mixins/orbital'/spec['helper']).read_text(encoding='utf8')
    function=re.search(r'\bfn\s+(\w+)\s*\(',helper).group(1)
    call=function+'('+','.join(b['value'] for b in spec['bindings'])+')'
    if spec.get('cancellable'):
        call='if let core::ops::ControlFlow::Break(result) = '+call+' { return result; }'
    else: call+=';'
    insertion='\n// WYR embedded Orbital: '+spec['id']+'\n'+helper+'\n'+call+'\n'
    write(file,source[:start]+insertion+source[start:])

def apply(root):
    stage=Path(root).resolve()
    if stage in (BASE/'horizon',SOURCE): raise RuntimeError('Owned stage required')
    bundle=SOURCE/'native/mixins/orbital'
    manifest=tomllib.loads((bundle/'horizon-mixin.toml').read_text(encoding='utf8'))
    hooks={h['name']:h for h in json.loads((stage/'native-hooks.json').read_text(encoding='utf8'))['hooks']}
    engine(stage)
    glue=(bundle/'glue.rs').read_text(encoding='utf8')
    # One user-visible mod switch controls WYR, Upgrader and Orbital together.
    glue=glue.replace('game.native_on(ID)', 'game.native_on("wyr")')
    gate='if permission < CONSOLE_LEVEL && !crate::game::wyr::catastrophes::authorized_owner(game,player) {'
    glue=replace_once(glue,'if permission < CONSOLE_LEVEL {',gate)
    glue=replace_once(glue,'!horizon_protocol::mod_list::local().iter().any(|m| m.id == ID)', '!horizon_protocol::mod_list::local().iter().any(|m| m.id == "wyr")')
    glue+='''
pub(in crate::game) fn wyr_start(game:&mut Game<'_>, number:u8, at:[f64;3])->bool {
    if !running(game) {return false;}
    snapshot(game);
    let fired=orbital_laser::wyr_force(number,at);
    show_output(game);
    fired
}
'''
    append_module(stage/'crates/horizon-client/src/game.rs','orbital_glue',glue)
    game_file=stage/'crates/horizon-client/src/game.rs'
    current=game_file.read_text(encoding='utf8')
    for old in ('if permission < CONSOLE_LEVEL {','if permission < CONSOLE_LEVEL && !crate::game::wyr::catastrophes::authorized_owner(player) {'):
        current=current.replace(old,gate)
    write(game_file,current)
    append_module(stage/'crates/horizon-client-render/src/lib.rs','orbital_post',post_source(),True)
    append_module(stage/'crates/horizon-client/src/client_lib.rs','orbital_carry',(bundle/'carry.rs').read_text(encoding='utf8'),True)
    # No orbital command, item key or wheel is exposed in the combined mod.
    active={'tick','frame','net_from_client','net_from_server','level_tick_post',
            'camera_shake','world_drawn','render_needs','render_frame','carry_tick',
            'client_level_load','client_level_unload'}
    for spec in manifest['injection']:
        if spec['id'] in active: inject(stage,spec,hooks)
    return {'orbital_commit':__import__('subprocess').check_output(['git','-C',str(SOURCE),'rev-parse','HEAD'],text=True).strip(),'catastrophes':6}

if __name__=='__main__':
    print(apply(Path(sys.argv[1])))
