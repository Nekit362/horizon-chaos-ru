"""Assemble the owned source stage; never modifies the upstream checkout."""
import argparse,importlib.util,pathlib,subprocess,sys
ROOT=pathlib.Path(__file__).resolve().parent.parent

def run_module(name,stage):
    file=ROOT/'fork'/f'{name}.py'
    spec=importlib.util.spec_from_file_location(name,file)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    module.apply(stage)

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--stage',default='modded-resumed');parser.add_argument('--visual',action='store_true');parser.add_argument('--orbital',action='store_true')
    args=parser.parse_args();stage=(ROOT/args.stage).resolve()
    if stage==ROOT/'horizon' or ROOT not in stage.parents:raise ValueError('Only an owned stage can be modified')
    subprocess.run([sys.executable,str(ROOT/'configure_hooks.py')],check=True)
    subprocess.run([sys.executable,str(ROOT/'build_port.py'),'--fork'],check=True)
    subprocess.run([sys.executable,str(ROOT/'refresh_stage.py'),str(stage)],check=True)
    game=stage/'crates/horizon-client/src/game.rs'
    text=game.read_text(encoding='utf8');a=text.index('pub(crate) mod upgrader_hardcore {');b=text.index('pub(crate) mod wyr {',a)
    upgrader=text[a:b].replace('game.native_on(ID)','game.native_on("wyr")').replace('"upgrader:upgrader"','"wyr:upgrader"')
    # Keep its distinct screen/attachment IDs; register a single visible mod.
    start=upgrader.index('pub(in crate::game) fn mods(') if 'pub(in crate::game) fn mods(' in upgrader else upgrader.index('pub fn mods(')
    brace=upgrader.index('{',start);depth=1;i=brace+1
    while depth:
        if upgrader[i]=='{':depth+=1
        elif upgrader[i]=='}':depth-=1
        i+=1
    upgrader=upgrader[:brace+1]+' let _=mods; '+upgrader[i-1:]
    game.write_text(text[:a]+upgrader+text[b:],encoding='utf8')
    path=stage/'crates/horizon-world/src/lib.rs'
    text=path.read_text(encoding='utf8');marker='pub mod wyr_shared {'
    if marker not in text:raise RuntimeError('Stage must first be produced by horizon-mixins apply')
    text=text[:text.index(marker)]+marker+'\n'+(ROOT/'port/src/shared.rs').read_text(encoding='utf8')+'\n}\n'
    if path.read_text(encoding='utf8')!=text:path.write_text(text,encoding='utf8')
    run_module('gameplay_patches',stage)
    if args.visual:
        gfx=stage/'crates/horizon-client-render/src/gfx.rs'
        text=gfx.read_text(encoding='utf8');active='horizon_world::wyr_shared::client_count("unsupported_transparency") > 0'
        if '// WYR visual patch v1' in text:
            text=text.replace('let pack_terrain = !frame.title\n','let pack_terrain = !frame.title && !('+active+')\n')
            text=text.replace('.filter(|_| !frame.title)\n            .and_then(|n| Some((n.material.as_ref()?, &n.frame.group)))','.filter(|_| !frame.title && !('+active+'))\n            .and_then(|n| Some((n.material.as_ref()?, &n.frame.group)))')
            gfx.write_text(text,encoding='utf8')
        run_module('visual_patches',stage)
    if args.orbital:run_module('orbital_integration',stage)
    print('Assembled',stage)

if __name__=='__main__':main()
