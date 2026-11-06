"""Rebuild the native bundle and the content mod from the local sources (Python 3.9+)."""
import hashlib
import json
from pathlib import Path
import zipfile

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "mixins" / "upgrader_hardcore"
DIST = ROOT / "dist"
REV = "3a817511807aa1056cfcf2598a05180737751cc1"
MIXINS = "5a64af938ec86b00d53d16a408aabc0038fb3e57"
BUNDLE.mkdir(parents=True, exist_ok=True)
DIST.mkdir(exist_ok=True)

def entry(archive, name, data):
    info=zipfile.ZipInfo(name,(2026,11,6,0,0,0))
    info.compress_type=zipfile.ZIP_DEFLATED
    info.external_attr=0o100644<<16
    archive.writestr(info,data)

core = (ROOT / "src/core.rs").read_text(encoding="utf-8")
engine = (ROOT / "src/engine.rs").read_text(encoding="utf-8")
(BUNDLE / "runtime.rs").write_text("mod core {\n" + core + "\n}\n" + engine, encoding="utf-8")

manifest = f'''[mod]
id = "upgrader_hardcore"
version = "0.1.0"
priority = 100

[compatibility]
horizon_revision = "{REV}"

[[module]]
id = "runtime"
file = "crates/horizon-client/src/game.rs"
name = "upgrader_hardcore"
source = "runtime.rs"
'''

G = ("game", "&mut crate::game::Game<'_>", "&mut *self")
R = ("game", "&crate::game::Game<'_>", "&*self")
S = "&crate::game::native_hooks::NativeScreenView<'_>"
I = "&horizon_client::items::ItemStack"
def inject(name, bindings, call):
    global manifest
    target = "native_" + name
    manifest += '\n[[injection]]\nid = "' + name + '"\nkind = "HEAD"\ntarget = "horizon_client::game::native_hooks::Game<\'_>::' + target + '"\nhelper = "' + name + '.rs"\nbindings = [\n'
    manifest += "".join('  { name = ' + json.dumps(n) + ', ty = ' + json.dumps(t) + ', value = ' + json.dumps(v) + ' },\n' for n,t,v in bindings)
    manifest += "]\n"
    args=", ".join(n+": "+t for n,t,v in bindings)
    (BUNDLE/(name+".rs")).write_text("fn upgrader_"+name+"("+args+") { crate::game::upgrader_hardcore::"+call+"; }\n",encoding="utf-8")

inject("mods",[("mods","&mut Vec<crate::game::native_hooks::NativeMod>","&mut *mods")],"mods(mods)")
inject("player_tick",[G],"tick(game)")
inject("server_data_loaded",[],"reload()")
inject("frame",[G],"frame(game)")
inject("key",[G,("key","&str","key"),("pressed","bool","pressed")],"key(game,key,pressed)")
inject("use_item",[G,("stack",I,"stack"),("claimed","&mut bool","&mut *claimed")],"use_device(game,stack,claimed)")
inject("use_item_on",[G,("stack",I,"stack"),("claimed","&mut bool","&mut *claimed")],"use_device(game,stack,claimed)")
inject("net_from_client",[("player","u128","player"),("channel","&str","channel"),("data","&[u8]","data")],"from_client(player,channel,data)")
inject("net_from_server",[("channel","&str","channel"),("data","&[u8]","data")],"from_server(channel,data)")
inject("gui_render_post",[R,("batch","&mut horizon_client::hud::HudBatch","&mut *batch")],"draw(game,batch)")
inject("screen_mouse_button_pressed_pre",[R,("screen",S,"screen"),("button","i32","button"),("canceled","&mut bool","&mut *canceled")],"mouse(game,screen,button,canceled)")
inject("screen_key_pressed_pre",[R,("key","&str","key"),("canceled","&mut bool","&mut *canceled")],"screen_key(game,key,canceled)")
inject("screen_char_typed_pre",[R,("c","char","c"),("canceled","&mut bool","&mut *canceled")],"typed(game,c,canceled)")
(BUNDLE/"horizon-mixin.toml").write_text(manifest,encoding="utf-8")
compat={"game":"26.3","hooks":410,"horizon_repository":"https://github.com/horizon-rust/horizon","horizon_revision":REV,"horizon_mixins_repository":"https://github.com/horizon-rust/horizon-mixins","horizon_mixins_revision":MIXINS}
(ROOT/"compatibility.json").write_text(json.dumps(compat,indent=2)+"\n",encoding="utf-8")
native={"schema":1,"id":"upgrader_hardcore","version":"0.1.0","game":"26.3","horizon":REV,"mixins":MIXINS}
# Direct game-field access intentionally pins this mod to the tested source revision.
with zipfile.ZipFile(DIST/"upgrader-hardcore-0.1.0.hnmod","w",compression=zipfile.ZIP_DEFLATED) as z:
    entry(z,"horizon-native.json",(json.dumps(native,indent=2)+"\n").encode())
    for p in sorted(BUNDLE.glob("*")):
        if p.suffix in (".rs",".toml"):entry(z,"mixins/upgrader_hardcore/"+p.name,p.read_bytes())

content=ROOT/"content"
source=Path(r"C:\Users\nikit\Downloads\upgrader-forge-1.3.0-26.1.2.jar")
with zipfile.ZipFile(source) as jar:
    for name in jar.namelist():
        if name.startswith("assets/upgrader/") and not name.endswith("/"):
            dest=content/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(jar.read(name))
    recipe=content/"data/upgrader/recipe/upgrader.json"
    recipe.parent.mkdir(parents=True,exist_ok=True)
    recipe.write_bytes(jar.read("data/upgrader/recipe/upgrader.json"))
item=content/"data/upgrader/horizon/item/upgrader.json"
item.parent.mkdir(parents=True,exist_ok=True)
item.write_text(json.dumps({"template":"minecraft:compass","max_stack_size":1}),encoding="utf-8")
(content/"horizon-mod.toml").write_text('id = "upgrader"\nname = "Апгрейдер — предмет"\nversion = "0.1.0"\ndescription = "Предмет для нативного Апгрейдера: Хардкор"\nauthors = ["EXEcheINZ (original mod)", "Local Horizon port"]\napi = "0.2"\nside = "both"\ncapabilities = ["registries"]\n',encoding="utf-8")
with zipfile.ZipFile(DIST/"upgrader-content-0.1.0.hmod","w",compression=zipfile.ZIP_DEFLATED) as z:
    for p in sorted(content.rglob("*")):
        if p.is_file():entry(z,p.relative_to(content).as_posix(),p.read_bytes())
with zipfile.ZipFile(DIST/"Upgrader-Horizon-RU-Hardcore-0.1.0.zip","w",compression=zipfile.ZIP_DEFLATED) as z:
    for name in ["upgrader-content-0.1.0.hmod","upgrader-hardcore-0.1.0.hnmod"]:
        entry(z,name,(DIST/name).read_bytes())
    for name in ["README-RU.md","VALIDATION.md"]:
        if (ROOT/name).exists():entry(z,name,(ROOT/name).read_bytes())
    for name in ["roulette-win.gif","roulette-outcomes.png"]:
        if (ROOT/"preview"/name).exists():entry(z,"preview/"+name,(ROOT/"preview"/name).read_bytes())
for p in sorted(DIST.iterdir()):
    if p.is_file():print(p.name,p.stat().st_size,hashlib.sha256(p.read_bytes()).hexdigest())
