"""Small isolated patcher smoke test; does not copy/build the game."""
from pathlib import Path
import importlib.util
import shutil
import hashlib

folder = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("visual_patches", folder.parent / "visual_patches.py")
patch = importlib.util.module_from_spec(spec)
spec.loader.exec_module(patch)
root = folder / "probe_source"
source = folder.parents[1] / "horizon"
files = {
    patch.RENDER: ["lib.rs", "post_chain.wgsl", "post_chain.rs", "ui.rs", "graphics_style.rs", "widgets.rs", "gfx.rs", "terrain.wgsl", "entity.wgsl", "visbuffer.wgsl"],
    patch.CLIENT: ["game.rs", "mobs/anim.rs", "game/moving_pistons.rs"],
}
for prefix, names in files.items():
    for name in names:
        target = root / (prefix + name)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source / (prefix + name), target)
result = patch.apply(root)
def hashes():
    return {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in root.rglob("*") if p.is_file()}
before = hashes()
patch.apply(root)
assert before == hashes(), "second apply must be idempotent"
print("PASS: all source anchors, 14 output files, second-apply idempotence")
print(result)
