"""Refresh only the generated modules in our own staged checkout, never upstream."""
import pathlib,sys
ROOT=pathlib.Path(__file__).resolve().parent
path=ROOT/(sys.argv[1] if len(sys.argv)>1 else 'modded')/'crates/horizon-client/src/game.rs'
text=path.read_text(encoding='utf8')
start=text.index('pub(crate) mod upgrader_hardcore {')
text=text[:start]
for name,source in [('upgrader_hardcore',ROOT.parent/'UpgraderHorizon/mixins/upgrader_hardcore/runtime.rs'),('wyr',ROOT/'port/mixins/wyr/runtime.rs')]:
    text+='pub(crate) mod '+name+' {\n'+source.read_text(encoding='utf8')+'\n}\n'
path.write_text(text,encoding='utf8')
print('Updated both staged runtime modules')
