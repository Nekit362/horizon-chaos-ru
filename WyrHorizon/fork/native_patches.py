# -*- coding: utf-8 -*-
"""Emit ordinary horizon-mixins HEAD/module fragments, never edit engine source.

assemble(destination, source_root=None) returns a TOML fragment and content bytes.
The temporary sparse copy is used only to read the already verified fork deltas.
"""
from pathlib import Path
import importlib.util
import json
import re
import shutil
import tempfile

BASE = Path(__file__).resolve().parent.parent
C = "crates/horizon-client/src/"
R = "crates/horizon-client-render/src/"


def masked(text):
    out = list(text)
    i = 0
    while i < len(text):
        start = i
        if text.startswith("//", i):
            i = text.find("\n", i)
            if i < 0: i = len(text)
        elif text.startswith("/*", i):
            depth = 1; i += 2
            while depth and i < len(text):
                if text.startswith("/*", i): depth += 1; i += 2
                elif text.startswith("*/", i): depth -= 1; i += 2
                else: i += 1
        elif text[i]=='r' and re.match(r'r#*"', text[i:i+20]):
            m = re.match(r'r(#+)?"', text[i:]); end = '"' + (m[1] or '')
            found = text.find(end, i + m.end()); i = len(text) if found < 0 else found + len(end)
        elif text[i] == '"':
            i += 1
            while i < len(text):
                if text[i] == '\\': i += 2
                elif text[i] == '"': i += 1; break
                else: i += 1
        elif text[i] == "'" and re.match(r"'(?:\\.|[^'\\])'", text[i:]):
            i += re.match(r"'(?:\\.|[^'\\])'", text[i:]).end()
        else:
            i += 1; continue
        for n in range(start, i):
            if out[n] != '\n': out[n] = ' '
    return ''.join(out)


def matching(text, at, opening='(', closing=')'):
    depth = 0
    for i in range(at, len(text)):
        if text[i] == opening: depth += 1
        elif text[i] == closing:
            depth -= 1
            if depth == 0: return i
    raise ValueError("Unbalanced Rust source")


def function(text, name, occurrence=0):
    clean = masked(text)
    found = list(re.finditer(r'\bfn\s+' + re.escape(name) + r'\s*(?:<[^\n]*>)?\s*\(', clean))
    if len(found) <= occurrence: raise ValueError('Missing function ' + name)
    match = found[occurrence]; at = clean.index('(', match.start()); end = matching(clean, at)
    brace = clean.index('{', end); finish = matching(clean, brace, '{', '}')
    result = text[end + 1:brace].strip()
    if result.startswith('->'): result = result[2:].strip()
    elif result: raise ValueError('Unsupported signature '+result)
    else: result = '()'
    return dict(start=match.start(), args=text[at + 1:end], result=result, body=text[brace + 1:finish], end=finish + 1)


def split_args(args):
    result=[]; start=0; depth=0
    for i,c in enumerate(args):
        if c in '([<': depth+=1
        elif c in ')]>': depth-=1
        elif c==',' and depth==0:
            if args[start:i].strip():result.append(args[start:i].strip())
            start=i+1
    if args[start:].strip():result.append(args[start:].strip())
    return result


def assemble(destination, source_root=None):
    source_root = Path(source_root or BASE/'horizon').resolve()
    destination = Path(destination); destination.mkdir(parents=True, exist_ok=True)
    fragments=[]; files=[]; content={}; gaps=[]

    def write(name, value):
        (destination/name).write_text(value,encoding='utf8'); files.append(name)

    def module(ident, file, name, source, public=False):
        payload='native_'+ident+'.rs'; write(payload,source)
        fragments.append(f'\n[[module]]\nid = "native_{ident}"\nfile = {json.dumps(file)}\nname = {json.dumps(name)}\nsource = {json.dumps(payload)}\nvisibility = "'+('pub' if public else 'pub(crate)')+'"\n')

    def head(ident, target, bindings, code, cancellable=False):
        payload='native_'+ident+'.rs'
        write(payload,code)
        # Run cancellation after other modules' same-priority HEAD callbacks,
        # notably Orbital's frame snapshot on Renderer::render.
        injection=('zz_native_' if cancellable else 'native_')+ident
        fragments.append(f'\n[[injection]]\nid = "{injection}"\nkind = "HEAD"\ntarget = {json.dumps(target)}\nhelper = {json.dumps(payload)}\ncancellable = '+str(cancellable).lower()+'\nbindings = [\n'+''.join('  { name = '+json.dumps(n)+', ty = '+json.dumps(t)+', value = '+json.dumps(v)+' },\n' for n,t,v in bindings)+']\n')

    def wrapper(ident, file, name, owner=None, body=None, omit=(), prefix='', suffix='', occurrence=0):
        old=function((source_root/file).read_text(encoding='utf8'),name,occurrence)
        body=body if body is not None else old['body']
        bindings=[]; params=[]; copies=[]
        for arg in split_args(old['args']):
            if 'self' in arg and ':' not in arg:
                mutable='mut' in arg; ty=('&mut ' if mutable else '&')+owner
                bindings.append(('this',ty,'&mut *self' if mutable else 'self')); params.append('this: '+ty)
                body=re.sub(r'\bself\b','this',body)
                continue
            pattern,ty=arg.split(':',1); pattern=pattern.strip(); ty=ty.strip()
            n=re.sub(r'^(?:mut|ref)\s+','',pattern)
            if n in omit:continue
            if not re.fullmatch(r'[A-Za-z_]\w*',n):raise ValueError('Destructured target '+file+'::'+name)
            if 'impl ' in ty:raise ValueError('Opaque binding '+n)
            value='&mut *'+n if ty.startswith('&mut ') else n
            if not ty.startswith('&'):
                # Bind an explicit borrow (the schema rejects method calls in
                # binding expressions), then clone inside ordinary helper code.
                value='&'+n
                bindings.append((n,'&'+ty,value));params.append(n+': &'+ty)
                copies.append('let '+pattern+' = '+n+'.clone();')
            else:
                bindings.append((n,ty,value)); params.append(pattern+': '+ty)
        result=old['result']
        if owner:
            body=re.sub(r'\bSelf\b',owner.split('<')[0],body); result=re.sub(r'\bSelf\b',owner,result)
        if 'Self' in old['result']:raise ValueError('Cancellable HEAD cannot target Self result: '+name)
        target=file[:-3].replace('crates/horizon-client/src/','horizon_client/').replace('crates/horizon-client-render/src/','horizon_client_render/').replace('/','::')
        if owner:target+='::'+owner
        target+='::'+name
        if file.startswith(C+'mobs/'):
            target=target.replace('horizon_client::','horizon_client_lib::',1)
        code='fn wyr_native_'+ident+'('+', '.join(params)+')->core::ops::ControlFlow<'+result+'> {\n'+'\n'.join(copies)+'\n'+prefix+'\nlet result=(|| -> '+result+' {\n'+body+'\n})();\n'+suffix+'\ncore::ops::ControlFlow::Break(result)\n}\n'
        head(ident,target,bindings,code,True)

    names=[C+'game/mounts.rs',C+'mobs/frog.rs',C+'motion.rs',R+'collisions.rs',C+'mobs/equine.rs',C+'mobs/ai.rs',C+'game/boats.rs',C+'game/creatures.rs',C+'game/lan_actions.rs',R+'lib.rs',R+'post_chain.wgsl',R+'post_chain.rs',C+'game.rs',R+'ui.rs',R+'graphics_style.rs',R+'widgets.rs',C+'mobs/anim.rs',R+'gfx.rs',R+'terrain.wgsl',R+'visbuffer.wgsl',C+'game/moving_pistons.rs',R+'entity.wgsl']
    with tempfile.TemporaryDirectory(prefix='wyr-native-reference-') as scratch:
        scratch=Path(scratch)
        for name in names:
            path=scratch/name;path.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source_root/name,path)
        for script in ('gameplay_patches','visual_patches'):
            spec=importlib.util.spec_from_file_location(script,BASE/'fork'/f'{script}.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);m.apply(scratch)
        patched={name:(scratch/name).read_text(encoding='utf8') for name in names}
        helpers=(scratch/(R+'wyr_visuals.rs')).read_text(encoding='utf8')

    def changed(ident,file,name,owner=None,**kwargs):
        wrapper(ident,file,name,owner,body=function(patched[file],name)['body'],**kwargs)

    # Gameplay: clone only the changed ordinary function bodies into cancellable HEAD.
    changed('enderman_seat',C+'game/mounts.rs','seat')
    changed('enderman_dismount',C+'game/mounts.rs','dismount_mount',"Game<'_>")
    frog=(BASE/'fork/frog_player_goal.rs').read_text(encoding='utf8').replace('struct WyrEatPlayer','pub(super) struct WyrEatPlayer').replace('super::PlayerView','crate::mobs::PlayerView').replace('super::ai::','crate::mobs::ai::')
    module('frog_goal',C+'mobs/frog.rs','wyr_native_frog','use super::*;\n'+frog)
    wrapper('frog_register',C+'mobs/frog.rs','register',body=function(patched[C+'mobs/frog.rs'],'register')['body'].replace('WyrEatPlayer::','wyr_native_frog::WyrEatPlayer::'))
    head('web',"horizon_client_lib::motion::MotionWorld<'_,'_,'_>::web_inside",[], 'fn wyr_native_web()->core::ops::ControlFlow<()> {if horizon_world::wyr_shared::current_count("no_cobweb_collision")>0 {core::ops::ControlFlow::Break(())}else{core::ops::ControlFlow::Continue(())}}',True)
    changed('fast_pig',C+'mobs/equine.rs','ridden_step')
    changed('spider',C+'mobs/ai.rs','ai_step')
    changed('sink_boat',C+'game/boats.rs','tick_boat',"Game<'_>")
    # Identify the explosion-event function from the unique source anchor.
    creature=patched[C+'game/creatures.rs']; marker=creature.index('let result = if species==Species::Creeper')
    candidate=[m.group(1) for m in re.finditer(r'\bfn\s+(\w+)\s*\(',masked(creature[:marker]))][-1]
    changed('creeper_flowers',C+'game/creatures.rs',candidate,"Game<'_>")
    collision_type="<SceneCollisions<'_,'_> as BlockCollisionWorld>"
    head('shovel_collision','horizon_client_render::collisions::'+collision_type+'::context_shape',
         [('block','&CollisionBlock','&block'),('context',"&CollisionContext<'_>",'context')],
         '''fn wyr_native_shovel(block:&CollisionBlock,context:&CollisionContext<'_>)->core::ops::ControlFlow<io::Result<VoxelShape>> {
let state=block.state.0;
if context.entity().is_some_and(|e|e.0==0) && horizon_world::wyr_shared::current_count("noclip_shovel_mineable_blocks")>0 && horizon_world::wyr_shared::shovel(state) {core::ops::ControlFlow::Break(Ok(VoxelShape::empty()))}else{core::ops::ControlFlow::Continue(())}}
''',True)
    # as_guest_with_host is type-generic; enter/leave are ordinary functions.
    module('actor_stack',C+'game/lan_actions.rs','wyr_native_actor','''thread_local! {static STACK:std::cell::RefCell<Vec<horizon_world::wyr_shared::ActorGuard>>=const{std::cell::RefCell::new(Vec::new())};}
pub(super) fn push(uuid:Option<u128>){STACK.with(|s|s.borrow_mut().push(horizon_world::wyr_shared::enter_actor(uuid)));}
pub(super) fn pop(){STACK.with(|s|{s.borrow_mut().pop();});}
''')
    wrapper('actor_enter',C+'game/lan_actions.rs','enter_guest',"Game<'_>",suffix='if result.is_some(){wyr_native_actor::push(this.native_acting_player().map(|p|p.uuid));}')
    head('actor_leave',"horizon_client::game::lan_actions::Game<'_>::leave_guest",[], 'fn wyr_native_leave(){wyr_native_actor::pop();}')

    # Shader flag, pipeline selection and the non-Clone GpuChecks move remain in
    # an ordinary new inherent method, not inside the transform's AST traversal.
    draw=function(patched[R+'gfx.rs'],'draw_frame')
    draw_body=draw['body'].replace('let vis_frame = flat_frame','let vis_frame = flat_frame\n            && horizon_world::wyr_shared::client_count("unsupported_transparency")==0')
    module('draw_frame',R+'gfx.rs','wyr_native_draw','use super::*;\nimpl Renderer { pub(super) fn wyr_native_draw_frame('+draw['args']+')->'+draw['result']+'{'+draw_body+'}\n}')
    for name in ('render','draw_scaled_frame'):
        original=function((source_root/(R+'gfx.rs')).read_text(encoding='utf8'),name)
        wrapper('route_'+name,R+'gfx.rs',name,'Renderer',body=original['body'].replace('self.draw_frame(','self.wyr_native_draw_frame('))
    # Entity source has a concrete String result and is safe to replace at HEAD.
    entity=patched[R+'entity.wgsl']
    wrapper('entity_shader',R+'gfx.rs','entity_shader',body='crate::overlay::wgsl_constants() + '+json.dumps(entity))
    # Cached device is sync even though Renderer::new is async. Preserve its cache.
    terrain_before=(source_root/(R+'terrain.wgsl')).read_text(encoding='utf8')
    terrain_after=patched[R+'terrain.wgsl']
    shader_body='''fn wyr_native_shader(on:&On<'_>, descriptor:&wgpu::ShaderModuleDescriptor<'_>)->core::ops::ControlFlow<wgpu::ShaderModule> {
let wgpu::ShaderSource::Wgsl(source)=&descriptor.source else{return core::ops::ControlFlow::Continue(());};
let source=source.replace("\\r\\n","\\n");
let original='''+json.dumps(terrain_before)+''';
if !source.contains(original) {return core::ops::ControlFlow::Continue(());}
let patched=source.replace(original,'''+json.dumps(terrain_after)+''');
let descriptor=wgpu::ShaderModuleDescriptor{label:descriptor.label,source:wgpu::ShaderSource::Wgsl(patched.into())};
let key=format!("{descriptor:?}");
core::ops::ControlFlow::Break(on.reuse.shaders.get(key,on.generation,||on.device.create_shader_module(descriptor)))
}
'''
    head('terrain_shader',"horizon_client_render::gfx::reuse::On<'_>::create_shader_module",[('on',"&On<'_>",'self'),('descriptor',"&wgpu::ShaderModuleDescriptor<'_>",'&descriptor')],shader_body,True)
    changed('moving_blocks',C+'game/moving_pistons.rs','moving_block_vertices',"Game<'_>")
    changed('hdr_mark',R+'gfx.rs','mark_translucent_terrain','Renderer')

    module('visual_state',R+'lib.rs','wyr_visuals',helpers,True)
    # No new PostEffects constructor/constant is needed: ship actual translated GLSL.
    requested=function(patched[C+'game.rs'],'requested_post_effects')['body'].replace('horizon_client::post_chain::wyr_random_effect()','horizon_client::wyr_visuals::random_effect()')
    # horizon-client's explicit render reexports do not reexport the new module;
    # inject a small forwarding module into its existing client_lib file.
    module('client_visual',C+'client_lib.rs','wyr_visuals','pub use horizon_client_render::wyr_visuals::*;',True)
    wrapper('post_request',C+'game.rs','requested_post_effects',"Game<'_>",body=requested)
    cache='''thread_local! {static POSES:std::cell::RefCell<std::collections::BTreeMap<u32,super::Poses>>=std::cell::RefCell::new(std::collections::BTreeMap::new());}
pub(super) fn load(id:u32,p:&mut super::Poses){POSES.with(|s|{if let Some(old)=s.borrow().get(&id){p.copy_from(old);}});}
pub(super) fn save(id:u32,p:&super::Poses){POSES.with(|s|{let mut s=s.borrow_mut();if s.len()>1024{s.clear();}s.entry(id).or_default().copy_from(p);});}
'''
    module('avatar',C+'mobs/anim.rs','wyr_native_poses',cache)
    frozen='''let player=&*m.kind.id=="minecraft:player";
let frozen=player && if m.id==0 {horizon_world::wyr_shared::client_count("no_animations")>0}else{horizon_world::wyr_shared::entity_count(m.id as i32,"no_animations")>0};
if frozen {wyr_native_poses::load(m.id,p);player_model_visibility(p,m.extra.hidden_model_parts,m.extra.spectator);return core::ops::ControlFlow::Break(());}
'''
    wrapper('freeze_player',C+'mobs/anim.rs','setup',prefix=frozen,suffix='if player {wyr_native_poses::save(m.id,p);}')
    # Standard buttons: action_button has concrete args; button_as's opaque
    # narration callback stays owned by the original and is dropped on cancellation.
    changed('action_button',R+'ui.rs','action_button')
    wrapper('button_as',R+'ui.rs','button_as',omit=('narration',),body='''let [x,y,w,h]=crate::wyr_visuals::jitter([x,y,w,h]);
button_with_room(batch,atlas,[x,y,w,h],label,active,hover,0.0,|rect|crate::widgets::Narratable::button(label.plain(),rect,active));
''')
    changed('hover',R+'ui.rs','hovered')
    for name in ('target_as','declare_action','end_frame'):
        changed('widgets_'+name,R+'widgets.rs',name)
    record=function((source_root/(R+'widgets.rs')).read_text(encoding='utf8'),'record')
    wrapper('widgets_record',R+'widgets.rs','record',body='let rect=crate::wyr_visuals::effective_rect(rect);\n'+record['body'])
    gaps.append('Flat graphics_style::button and per-screen cache reset have destructured/opaque signatures: native fragment preserves standard sprite/action button shaking; flat-style custom controls require additional caller hooks.')

    # Fully owned GLSL, consumed by the stock post_glsl translator at runtime.
    vertex='''#version 330
layout(location=0) out vec2 texCoord;
void main(){vec2 p=vec2(float((gl_VertexIndex<<1)&2),float(gl_VertexIndex&2));gl_Position=vec4(p*2.0-1.0,0.0,1.0);texCoord=p;}
'''
    content['assets/wyr/shaders/post/wyr_quad.vsh']=vertex.encode()
    expressions=[
        'vec3(1.0)-c.rgb',
        'vec3(dot(c.rgb,vec3(0.299,0.587,0.114)))',
        'c.rgb*(mod(floor(texCoord.y*float(textureSize(InSampler,0).y)),2.0)<1.0?1.0:0.45)',
        'floor(c.rgb*5.0+0.5)/5.0',
        'vec3(texture(InSampler,clamp(texCoord+vec2(4.0/float(textureSize(InSampler,0).x),0.0),vec2(0.0),vec2(1.0))).r,c.g,texture(InSampler,clamp(texCoord-vec2(4.0/float(textureSize(InSampler,0).x),0.0),vec2(0.0),vec2(1.0))).b)',
        'vec3(clamp(3.0*length(c.rgb-texture(InSampler,clamp(texCoord+vec2(step.x,0.0),vec2(0.0),vec2(1.0))).rgb)+3.0*length(c.rgb-texture(InSampler,clamp(texCoord+vec2(0.0,step.y),vec2(0.0),vec2(1.0))).rgb),0.0,1.0))',
    ]
    for number,expression in enumerate(expressions):
        fragment='#version 330\nuniform sampler2D InSampler;\nlayout(location=0) in vec2 texCoord;\nlayout(location=0) out vec4 fragColor;\nvoid main(){vec4 c=texture(InSampler,texCoord);vec2 step=1.0/vec2(textureSize(InSampler,0));fragColor=vec4('+expression+',c.a);}\n'
        content[f'assets/wyr/shaders/post/random_{number}.fsh']=fragment.encode()
        config={'targets':{'wyr:random_swap':{}},'passes':[
            {'vertex_shader':'wyr:post/wyr_quad','fragment_shader':f'wyr:post/random_{number}','inputs':[{'sampler_name':'In','target':'minecraft:main','bilinear':True}],'output':'wyr:random_swap'},
            {'vertex_shader':'minecraft:core/screenquad','fragment_shader':'minecraft:post/blit','inputs':[{'sampler_name':'In','target':'wyr:random_swap','bilinear':True}],'output':'minecraft:main','uniforms':{'BlitConfig':[{'type':'vec4','value':[1,1,1,1]}]}}
        ]}
        content[f'assets/wyr/post_effect/random_{number}.json']=(json.dumps(config,indent=2)+'\n').encode()
    for path,data in content.items():
        target=destination/'content-assets'/path;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data)
    fragment=''.join(fragments);write('native-fragment.toml',fragment)
    report={'manifest':fragment,'files':files,'content':content,'gaps':gaps,'source_revision':'3a817511807aa1056cfcf2598a05180737751cc1'}
    write('native-report.json',json.dumps({k:v for k,v in report.items() if k not in ('manifest','content')},indent=2))
    return report


emit=assemble
if __name__=='__main__':
    import sys
    result=assemble(Path(sys.argv[1]))
    print(json.dumps({'files':len(result['files']),'content_assets':len(result['content']),'gaps':result['gaps']},indent=2))
