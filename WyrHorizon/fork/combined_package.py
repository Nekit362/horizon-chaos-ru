# -*- coding: utf-8 -*-
"""Deterministic personal WYR + Upgrader + Orbital content/source delivery.

This content package REQUIRES the separately patched Horizon engine. It is not
a native bundle and cannot supply game logic to unmodified Horizon.
"""
from pathlib import Path, PurePosixPath
import argparse
import hashlib
import json
import subprocess
import zipfile

BASE = Path(__file__).resolve().parent.parent
DEFAULT_UPGRADER = BASE.parent / "UpgraderHorizon"
HORIZON = "3a817511807aa1056cfcf2598a05180737751cc1"
MIXINS = "5a64af938ec86b00d53d16a408aabc0038fb3e57"
ORBITAL = "51468c5f69c8f61cb8982a44700853390eb23ed9"
MANIFEST = '''id = "wyr"
name = "Что бы ты выбрал? — Полная сборка"
version = "0.2.0"
description = "WYR, Апгрейдер: Хардкор и орбитальные катастрофы. Требуется отдельная сборка движка Horizon WYR."
authors = ["EightSidedSquare (WYR)", "EXEcheINZ (Upgrader)", "Stink-O (Orbital Laser, MIT)", "Local Horizon port"]
api = "0.2"
side = "both"
capabilities = ["registries"]
'''


def safe_name(name):
    path = PurePosixPath(name)
    if "\\" in name or path.is_absolute() or any(p in ("", ".", "..") for p in name.split("/")) or ":" in name:
        raise ValueError("Unsafe archive path: " + name)
    return path.as_posix()


def put(entries, folded, name, data, origin):
    name = safe_name(name)
    if name.casefold() in folded:
        raise ValueError(f"Asset collision: {name} from {origin}; already provided by {folded[name.casefold()]}")
    folded[name.casefold()] = origin
    entries[name] = data


def deterministic_zip(path, entries):
    path.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(path, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for name, data in sorted(entries.items()):
            info = zipfile.ZipInfo(safe_name(name), (2000, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.create_system = 3
            info.external_attr = 0o100644 << 16
            archive.writestr(info, data, compresslevel=9)
    return hashlib.sha256(path.read_bytes()).hexdigest()


def content_entries(upgrader=DEFAULT_UPGRADER, wyr_jar=None, upgrader_jar=None):
    entries, folded = {}, {}
    for label, directory, jar, namespace in (
        ("WYR", BASE / "port/content", wyr_jar, "wyr"),
        ("Upgrader", Path(upgrader) / "content", upgrader_jar, "upgrader"),
    ):
        # Optional supplied originals fill protected assets omitted from source delivery.
        # Port-owned overrides then intentionally replace that base inside ONE input.
        local = {}
        if jar:
            with zipfile.ZipFile(jar) as source:
                for name in source.namelist():
                    if name.startswith(f"assets/{namespace}/") and not name.endswith("/"):
                        local[safe_name(name)] = source.read(name)
                if namespace == "upgrader":
                    name = "data/upgrader/recipe/upgrader.json"
                    local[name] = source.read(name)
        for path in sorted(directory.rglob("*")):
            if path.is_symlink():
                raise ValueError("Symlink in content: " + str(path))
            if path.is_file():
                name = path.relative_to(directory).as_posix()
                if name.startswith(("assets/", "data/")):
                    local[name] = path.read_bytes()
        for name, data in local.items():
            put(entries, folded, name, data, label)
    orbital = BASE / "orbital-laser"
    for folder in ("orbital-laser-mod", "orbital-sounds"):
        for path in sorted((orbital / folder).rglob("*")):
            if path.is_symlink():
                raise ValueError("Symlink in Orbital content: " + str(path))
            if path.is_file():
                name = path.relative_to(orbital / folder).as_posix()
                if name.startswith(("assets/", "data/")):
                    put(entries, folded, name, path.read_bytes(), "Orbital/" + folder)
    # Fail early when source delivery has not been supplied the original WYR/Upgrader assets.
    for required in ("assets/wyr/sounds.json", "assets/upgrader/textures/item/upgrader.png", "assets/orbital/sounds.json"):
        if required not in entries:
            raise ValueError(f"Missing {required}; supply --wyr-jar and --upgrader-jar when rebuilding from source delivery")
    for name, data in entries.items():
        if name.endswith(".json"):
            json.loads(data.decode("utf-8"))
    entries["horizon-mod.toml"] = MANIFEST.encode("utf-8")
    entries["licenses/Orbital-Laser-MIT.txt"] = (orbital / "LICENSE").read_bytes()
    entries["licenses/Orbital-Laser-NOTICE.txt"] = (orbital / "NOTICE").read_bytes()
    return entries


def source_entries(upgrader=DEFAULT_UPGRADER):
    """Explicit allowlist: no JARs, decompiled originals, engine trees or targets."""
    entries = {}

    def add(path, name):
        path = Path(path)
        if path.is_symlink():
            raise ValueError("Symlink in source: " + str(path))
        if path.is_file():
            name = safe_name(name)
            if name in entries:
                raise ValueError("Source collision: " + name)
            entries[name] = path.read_bytes()

    def tree(directory, prefix, suffixes=None):
        for p in sorted(Path(directory).rglob("*")):
            if any(part in ("target", "__pycache__", ".git", "probe_source") for part in p.relative_to(directory).parts):
                continue
            if p.is_file() and (suffixes is None or p.suffix in suffixes):
                add(p, prefix + "/" + p.relative_to(directory).as_posix())

    for name in ("build_port.py", "configure_hooks.py", "refresh_stage.py", "upgrade_state.py", "translate.py", "write_docs.py", "ru_ru.json", "README-RU.md", "source-sha256.txt"):
        add(BASE / name, "WyrHorizon/" + name)
    for name in ("Cargo.toml", "Cargo.lock", "hooks.json", "extra-hooks.json", "coverage.json", "choice-definitions.json", "compatibility.json", "README-RU.md"):
        add(BASE / "port" / name, "WyrHorizon/port/" + name)
    for name in ("src", "mixins"):
        tree(BASE / "port" / name, "WyrHorizon/port/" + name, {".rs", ".toml"})
    for p in sorted((BASE / "fork").glob("*")):
        if p.suffix in (".py", ".rs", ".md"):
            add(p, "WyrHorizon/fork/" + p.name)
    tree(BASE / "fork/visual", "WyrHorizon/fork/visual", {".rs", ".wgsl", ".py", ".bat"})
    # Python 3.9 needs the already vendored MIT tomli parser used by Orbital integration.
    tree(BASE / "fork/python-deps", "WyrHorizon/fork/python-deps")
    for name in ("Cargo.toml", "Cargo.lock", "compatibility.json", "README-RU.md", "VALIDATION.md"):
        add(Path(upgrader) / name, "UpgraderHorizon/" + name)
    for name in ("src", "mixins", "tools"):
        tree(Path(upgrader) / name, "UpgraderHorizon/" + name, {".rs", ".py", ".toml"})
    # Port-owned registry/item definitions and Russian translations only.
    for project, content in (("WyrHorizon/port", BASE / "port/content"), ("UpgraderHorizon", Path(upgrader) / "content")):
        for p in sorted(content.rglob("*.json")):
            name = p.relative_to(content).as_posix()
            if "/horizon/" in name or name.endswith("/lang/ru_ru.json") or "/items/" in name:
                add(p, project + "/content/" + name)
    orbital = BASE / "orbital-laser"
    for name in ("LICENSE", "NOTICE", "README.md"):
        add(orbital / name, "WyrHorizon/orbital-laser/" + name)
    for name in ("mod/src", "mod/shaders", "native/engine-shim", "native/mixins/orbital", "orbital-laser-mod", "orbital-sounds"):
        tree(orbital / name, "WyrHorizon/orbital-laser/" + name)
    for name in ("build.rs", "Cargo.toml", "Cargo.lock", "horizon-mod.toml"):
        add(orbital / "mod" / name, "WyrHorizon/orbital-laser/mod/" + name)
    provenance = {
        "requires_custom_engine": True,
        "repositories": {
            "horizon": {"url": "https://github.com/horizon-rust/horizon.git", "commit": HORIZON},
            "horizon-mixins": {"url": "https://github.com/horizon-rust/horizon-mixins.git", "commit": MIXINS},
            "orbital-laser": {"url": "https://github.com/Stink-O/orbital-laser.git", "commit": ORBITAL, "license": "MIT"},
        },
        "omitted": ["original JARs", "decompiled reference", "Minecraft resources", "entire Horizon engine", "Rust toolchains", "targets", "credentials"],
    }
    entries["PROVENANCE.json"] = (json.dumps(provenance, ensure_ascii=False, indent=2) + "\n").encode()
    entries["SOURCE-README-RU.md"] = SOURCE_README.encode("utf-8")
    entries["bootstrap.py"] = BOOTSTRAP.encode("utf-8")
    return entries


SOURCE_README = '''# Исходники отдельной сборки WYR

WyrHorizon/port/src — WYR; UpgraderHorizon/src — казино; WyrHorizon/fork — патчеры отдельного движка. Orbital Laser включён под MIT с LICENSE/NOTICE, нативным shim, помощниками и WGSL. В PROVENANCE.json зафиксированы upstream commits для повторного получения зависимостей. Полного исходного движка, JAR, декомпиляции, toolchain и target здесь нет.

Ресурсы WYR/Upgrader для личной сборки берутся из ваших оригинальных JAR:

    py -3.9 WyrHorizon/fork/combined_package.py --wyr-jar PATH/wyr.jar --upgrader-jar PATH/upgrader.jar --output PATH/wyr-full-content.hmod

Исходники патчеров сами по себе не устанавливаются в обычный Horizon: сначала нужен checkout закреплённого Horizon и horizon-mixins, native staging, затем fork/build_fork.py с --visual --orbital и сборка изменённого движка. Эта выдача не содержит готового .hnmod и не обещает совместимость со стандартным лаунчером.

Генератор build_port.py использует port/choice-definitions.json с описаниями и свойствами вариантов; оригинальный Java для пересборки не требуется. Сгенерированные native bundles и catalog.rs также включены. bootstrap.py получает закреплённые исходники Horizon/horizon-mixins, создаёт native stage и применяет fork. Запуск: `py -3.9 bootstrap.py` (получение исходников) или `py -3.9 bootstrap.py --build` (также полная release-сборка; нужны Git, Cargo, Rust и системный C/C++ toolchain). Он не запускает игру. Повторную сборку делайте с новым `--stage`, старые непустые папки не перезаписываются.

Некоторые прежние диагностические/preview scripts сохраняют локальные пути к инструментам; они не входят в bootstrap flow. combined_package.py использует относительное расположение обоих проектов и принимает оригинальные JAR через параметры. Совместимость и фактическую проверку конкретной выдачи смотрите в сопровождающем отчёте: наличие скрипта пересборки не означает, что выполнен интерактивный тест игры.

Содержимое .hmod включает оригинальные ресурсы двух модов для личного использования. Публичная публикация этих ресурсов в этом проекте не выполнялась.
'''

BOOTSTRAP = '''# -*- coding: utf-8 -*-
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
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upgrader", type=Path, default=DEFAULT_UPGRADER)
    parser.add_argument("--output", type=Path, default=BASE / "dist/combined/wyr-full-content-0.2.0.hmod")
    parser.add_argument("--source-dir", type=Path)
    parser.add_argument("--source-zip", type=Path)
    parser.add_argument("--wyr-jar", type=Path)
    parser.add_argument("--upgrader-jar", type=Path)
    args = parser.parse_args()
    entries = content_entries(args.upgrader, args.wyr_jar, args.upgrader_jar)
    result = {"content": str(args.output), "sha256": deterministic_zip(args.output, entries), "entries": len(entries), "requires_custom_engine": True}
    if args.source_dir or args.source_zip:
        sources = source_entries(args.upgrader)
        if args.source_dir:
            if args.source_dir.exists() and any(args.source_dir.iterdir()):
                raise ValueError("Source delivery directory must be empty: " + str(args.source_dir))
            for name, data in sources.items():
                path = args.source_dir / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(data)
            result["source_dir"] = str(args.source_dir)
        if args.source_zip:
            result["source_zip"] = str(args.source_zip)
            result["source_sha256"] = deterministic_zip(args.source_zip, sources)
        result["source_entries"] = len(sources)
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
