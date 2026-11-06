"""Build schema-safe Orbital fragments for the ONE combined launcher .hnmod.

Call assemble(bundle_directory), append result['manifest'] to the combined
horizon-mixin.toml, and package the generated files under that same mod id.
No source checkout or installed game is patched. No build script ships.
"""
from pathlib import Path
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parent))
import orbital_integration as direct

BASE = direct.BASE
SOURCE = direct.SOURCE
REVISION = '3a817511807aa1056cfcf2598a05180737751cc1'
ACTIVE = {'tick', 'frame', 'net_from_client', 'net_from_server', 'level_tick_post',
          'camera_shake', 'world_drawn', 'render_needs', 'render_frame',
          'carry_tick', 'client_level_load', 'client_level_unload'}


def write(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding='utf8')


def rust_environment():
    env = os.environ.copy()
    cargo_home = BASE/'toolchain/cargo'
    rustup_home = BASE/'toolchain/rustup'
    if cargo_home.is_dir() and rustup_home.is_dir():
        env['CARGO_HOME'] = str(cargo_home)
        env['RUSTUP_HOME'] = str(rustup_home)
        env['RUSTUP_TOOLCHAIN'] = 'stable'
        env['PATH'] = str(cargo_home/'bin') + os.pathsep + env.get('PATH', '')
    return env


def assemble_engine(bundle, rustc=None):
    """Run upstream shader generator now, replacing all OUT_DIR/absolute paths."""
    destination = bundle/'orbital_engine'
    with tempfile.TemporaryDirectory(prefix='wyr-native-orbital-') as temp:
        stage = Path(temp)
        # Reuse the current forced API and shim exactly as the tested direct fork
        # does, but let it write only to this private temporary staging tree.
        for name in ('horizon-client', 'horizon-client-render'):
            write(stage/f'crates/{name}/Cargo.toml', '[dependencies]\n')
        direct.engine(stage)
        engine = stage/'crates/wyr-orbital-engine'
        generated = stage/'generated'
        generated.mkdir()
        env = rust_environment()
        compiler = str(rustc) if rustc else shutil.which('rustc', path=env['PATH'])
        if not compiler:
            raise RuntimeError('Rust compiler required to prebuild Orbital shaders')
        generator = stage/('shader-generator.exe' if os.name == 'nt' else 'shader-generator')
        subprocess.run([compiler, '--edition', '2024', '-O', str(engine/'build.rs'), '-o', str(generator)],
                       env=env, check=True, capture_output=True, text=True)
        env['CARGO_MANIFEST_DIR'] = str(engine)
        env['OUT_DIR'] = str(generated)
        subprocess.run([str(generator)], env=env, check=True, capture_output=True, text=True)

        passes = (generated/'passes.rs').read_text(encoding='utf8')
        def relative_include(match):
            # Rust Debug strings emitted by build.rs are valid JSON strings,
            # including escaped Windows separators.
            path = Path(json.loads(match.group(1)))
            if path.parent == generated:
                relative = '../data/gen/' + path.name
            elif path.parent == engine/'shaders':
                relative = '../data/shaders/' + path.name
            else:
                raise RuntimeError('Unexpected generated shader include: ' + str(path))
            return 'include_str!(' + json.dumps(relative) + ')'
        passes = re.sub(r'include_str!\(("(?:\\.|[^"\\])*")\)', relative_include, passes)
        write(engine/'gen/passes.rs', passes)
        lib = (engine/'src/lib.rs').read_text(encoding='utf8')
        lib = direct.replace_once(lib, 'include!(concat!(env!("OUT_DIR"), "/passes.rs"));',
                                  'include!("../gen/passes.rs");')
        write(engine/'src/lib.rs', lib)
        for path in (engine/'src').rglob('*.rs'):
            text = path.read_text(encoding='utf8')
            text = text.replace('include_str!("../../shaders/', 'include_str!("../../data/shaders/')
            write(path, text)

        # Copy only the permitted bundle crate inputs. The generator executable,
        # build.rs, Cargo.lock and temporary paths never enter the package.
        for name in ('src', 'gen'):
            shutil.copytree(engine/name, destination/name, dirs_exist_ok=True)
        shutil.copytree(engine/'shaders', destination/'data/shaders', dirs_exist_ok=True)
        for path in generated.glob('*.wgsl'):
            target = destination/'data/gen'/path.name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(path, target)
        shutil.copy2(engine/'Cargo.toml', destination/'Cargo.toml')
        # MIT notice stays under permitted data/, rather than a disallowed plain
        # LICENSE filename beside Cargo.toml.
        write(destination/'data/LICENSE-Orbital.txt', (SOURCE/'LICENSE').read_text(encoding='utf8'))

    for path in destination.rglob('*'):
        if not path.is_file():
            continue
        relative = path.relative_to(destination)
        if path.name in ('build.rs', 'Cargo.lock') or (path.suffix not in ('.rs', '.toml') and relative.parts[0] != 'data'):
            raise RuntimeError('Disallowed existing file in Orbital crate: ' + str(relative))
        if path.suffix == '.rs':
            text = path.read_text(encoding='utf8')
            if 'env!("OUT_DIR")' in text:
                raise RuntimeError('Unresolved build-time OUT_DIR: ' + str(relative))
            for match in re.finditer(r'include_str!\(("(?:\\.|[^"\\])*")\)', text):
                included = json.loads(match.group(1))
                if included.endswith('.wgsl') and '/data/' not in included:
                    raise RuntimeError('Shader outside data/: ' + str(relative) + ': ' + included)
                if not (path.parent/included).resolve().is_relative_to(destination.resolve()):
                    raise RuntimeError('Include leaves Orbital crate: ' + included)
                if not (path.parent/included).is_file():
                    raise RuntimeError('Missing included file: ' + included)
    return destination


def glue_source():
    bundle = SOURCE/'native/mixins/orbital'
    glue = (bundle/'glue.rs').read_text(encoding='utf8')
    glue = glue.replace('game.native_on(ID)', 'game.native_on("wyr")')
    glue = glue.replace('t.starts_with("Type /orbital give") && no_content(game)', 't.starts_with("Type /orbital give")')
    glue = direct.replace_once(glue, 'if permission < CONSOLE_LEVEL {',
        'if permission < CONSOLE_LEVEL && !crate::game::wyr::catastrophes::authorized_owner(game,player) {')
    glue = direct.replace_once(glue, '!horizon_protocol::mod_list::local().iter().any(|m| m.id == ID)',
        '!horizon_protocol::mod_list::local().iter().any(|m| m.id == "wyr")')
    glue += '''
pub(in crate::game) fn wyr_start(game:&mut Game<'_>, number:u8, at:[f64;3])->bool {
    if !running(game) {return false;}
    snapshot(game);
    let fired=orbital_laser::wyr_force(number,at);
    show_output(game);
    fired
}
'''
    return glue


def toml_value(value):
    if isinstance(value, bool):
        return str(value).lower()
    return json.dumps(value, ensure_ascii=False)


def assemble(bundle_directory, revision=REVISION, rustc=None):
    """Emit files and return TOML fragment; caller owns the single mod manifest."""
    if revision != REVISION:
        raise ValueError('This integration was verified against Horizon ' + REVISION)
    bundle = Path(bundle_directory).resolve()
    bundle.mkdir(parents=True, exist_ok=True)
    assemble_engine(bundle, rustc)
    upstream = SOURCE/'native/mixins/orbital'
    manifest = direct.tomllib.loads((upstream/'horizon-mixin.toml').read_text(encoding='utf8'))
    hooks = {h['name']: h for h in json.loads((BASE/'horizon/native-hooks.json').read_text(encoding='utf8'))['hooks']}
    fragment = '\n# Orbital engine and effects; included under the same combined mod ID.\n'
    fragment += '[[crate]]\nid = "orbital_engine"\npath = "orbital_engine"\n\n'
    modules = [
        ('orbital_glue', 'crates/horizon-client/src/game.rs', 'orbital_glue', glue_source(), False),
        ('orbital_post', 'crates/horizon-client-render/src/lib.rs', 'orbital_post', direct.post_source(), True),
        ('orbital_carry', 'crates/horizon-client/src/client_lib.rs', 'orbital_carry', (upstream/'carry.rs').read_text(encoding='utf8'), True),
    ]
    for identifier, file, name, source, public in modules:
        helper = identifier + '.rs'
        write(bundle/helper, source)
        fragment += '[[module]]\n' + ''.join(k+' = '+toml_value(v)+'\n' for k,v in
            [('id',identifier),('file',file),('name',name),('source',helper)])
        if public:
            fragment += 'visibility = "pub"\n'
        fragment += '\n'
    for spec in manifest['injection']:
        if spec['id'] not in ACTIVE:
            continue
        identifier = 'orbital_' + spec['id']
        helper = identifier + '.rs'
        write(bundle/helper, (upstream/spec['helper']).read_text(encoding='utf8'))
        target = hooks[spec['hook']]['target'] if 'hook' in spec else spec['target']
        fragment += '[[injection]]\n' + ''.join(k+' = '+toml_value(v)+'\n' for k,v in
            [('id',identifier),('kind',spec['kind']),('target',target),('helper',helper)])
        if spec.get('cancellable'):
            fragment += 'cancellable = true\n'
        fragment += 'bindings = [\n'
        for binding in spec['bindings']:
            fragment += '  { ' + ', '.join(k+' = '+toml_value(v) for k,v in binding.items()) + ' },\n'
        fragment += ']\n\n'
    # Deliberately no [mod] or [compatibility] header: the root manifest is the
    # one source of truth for id/version/revision across WYR/Upgrader/Orbital.
    write(bundle/'orbital-fragment.toml', fragment)
    files = [p for p in bundle.rglob('*') if p.is_file() and
             (p.relative_to(bundle).parts[0] == 'orbital_engine' or p.name.startswith('orbital_') or p.name == 'orbital-fragment.toml')]
    return {'manifest':fragment, 'horizon_revision':revision,
            'modules':3, 'injections':len(ACTIVE), 'crate':'orbital-laser',
            'files':len(files), 'bytes':sum(p.stat().st_size for p in files),
            'engine_source_commit':'51468c5f69c8f61cb8982a44700853390eb23ed9'}


if __name__ == '__main__':
    result = assemble(sys.argv[1])
    print(json.dumps({k:v for k,v in result.items() if k!='manifest'}, indent=2))
