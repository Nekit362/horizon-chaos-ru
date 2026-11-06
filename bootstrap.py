# -*- coding: utf-8 -*-
"""Fetch pinned engine sources; optionally build the separate WYR engine."""
from pathlib import Path
import argparse,json,shutil,subprocess,sys
ROOT=Path(__file__).resolve().parent
def run(*args,cwd=None):subprocess.run(list(map(str,args)),cwd=cwd,check=True)
def main():
    p=argparse.ArgumentParser();p.add_argument('--build',action='store_true');p.add_argument('--stage',default='full-engine');a=p.parse_args()
    wyr=ROOT/'WyrHorizon';pins=json.loads((ROOT/'PROVENANCE.json').read_text(encoding='utf8'))['repositories']
    for name in ('horizon','horizon-mixins'):
        dst=wyr/name;pin=pins[name]
        if not dst.exists():
            run('git','clone','--no-checkout',pin['url'],dst)
            run('git','checkout','--detach',pin['commit'],cwd=dst)
        actual=subprocess.check_output(['git','-C',str(dst),'rev-parse','HEAD'],text=True).strip()
        if actual!=pin['commit']:raise RuntimeError('Unexpected checkout revision: '+str(dst))
    print('Pinned upstream sources ready; Orbital MIT source is already supplied.')
    if not a.build:return
    stage=(wyr/a.stage).resolve()
    if wyr not in stage.parents:raise RuntimeError('Stage must be inside WyrHorizon')
    if stage.exists() and any(stage.iterdir()):raise RuntimeError('Use a fresh --stage directory')
    run(sys.executable,wyr/'configure_hooks.py')
    run(sys.executable,wyr/'build_port.py','--fork')
    mods=wyr/'combined-generated-mods';mods.mkdir(exist_ok=True)
    for src,dst in ((ROOT/'UpgraderHorizon/mixins/upgrader_hardcore',mods/'00_upgrader'),(wyr/'port/mixins/wyr',mods/'10_wyr')):
        shutil.copytree(src,dst,dirs_exist_ok=True)
    mixins=wyr/'horizon-mixins'
    run('cargo','fetch','--locked',cwd=mixins)
    run('cargo','build','--locked','-p','horizon-mixins-cli',cwd=mixins)
    run('cargo','fetch','--locked',cwd=wyr/'horizon')
    tool=mixins/'target/debug'/('horizon-mixins.exe' if sys.platform=='win32' else 'horizon-mixins')
    run(tool,'apply','--workspace',wyr/'horizon','--mods',mods,'--out',stage)
    run(sys.executable,wyr/'fork/build_fork.py','--stage',a.stage,'--visual','--orbital')
    # Fork integration adds the vendored orbital workspace member to Cargo.lock.
    run('cargo','build','--release','-p','horizon-client',cwd=stage)
    print('Built separate engine in '+str(stage/'target/release'))
if __name__=='__main__':main()
