import pathlib, re, json, zipfile, hashlib,sys

ROOT = pathlib.Path(__file__).resolve().parent
PORT = ROOT / 'port'
BUNDLE = PORT / 'mixins/wyr'
REV = '3a817511807aa1056cfcf2598a05180737751cc1'
SUPPORTED = set('''dummy unlimited_bacon elytra bundle invisibility double_mining_speed jump_height_increase
explosion_immunity fall_damage_immunity fire_immunity edible_leather diamond_ore_drops smelt_inventory speed_increase
keep_inventory enchant_inventory remove_negative_choice short knockback set_items_to_full_stack
unbreakable_tools all_advancements faster_climbing extra_heart healed_by_cactus total_shield_blocking
remove_chunk randomized_block_drops hotbar_slot_fov hide_health adventure_mode_in_darkness
falling_step_blocks leaves_to_creepers flower_allergy teleport_to_world_height teleport_to_overworld
teleport_to_nether evaporate_water beds_always_explode projectiles_instantly_kill inedible_meat
armor_falls_off_when_jumping disable_crafting_tables nausea_near_fish double_block_item_depletion
thirst_bar offset_crosshair babify_zombies e_block_damage remove_positive_choice set_items_to_single blinded_by_sunlight
inverted_movement only_pick_negative_choices no_side_block_placement no_fence_gate_opening'''.split())

def main():
    lang = json.loads((ROOT/'ru_ru.json').read_text(encoding='utf8'))
    extras=json.loads((PORT/'extra-hooks.json').read_text(encoding='utf8')) if (PORT/'extra-hooks.json').exists() else {'implemented_ids':[]}
    supported=SUPPORTED | set(extras['implemented_ids']) | set('mvp treasure_fishing_loot instant_taming cheap_trades egg_ender_pearl piglins_barter_ender_eyes explode'.split())
    defs = json.loads((PORT/'choice-definitions.json').read_text(encoding='utf8'))
    catalog = []
    for definition in defs:
        id=definition['id'];positive=definition['positive'];persistent=definition['persistent'];stackable=definition['stackable']
        ru = lang['choice.wyr.'+id+'.description']
        random=id not in ('dummy','unlimited_games','no_more_video_games','no_games')
        catalog.append(dict(id=id,ru=ru,positive=positive,persistent=persistent,stackable=stackable,supported=(id in supported or not random or '--fork' in sys.argv),random=random))
    assert len(catalog) == 83, len(catalog)
    for number,name in enumerate(('Чёрная дыра','Метеоритный шторм','Убийца планет','Жезл Бога','Провал','Боевой заряд'),1):
        catalog.append(dict(id=f'catastrophe_{number}',ru=name,positive=False,persistent=False,stackable=False,supported=True,random=True))
    raw = 'pub static CATALOG: &[Choice] = &[\n'+''.join('Choice { '+', '.join(k+': '+(json.dumps(v,ensure_ascii=False) if isinstance(v,str) else str(v).lower()) for k,v in row.items())+' },\n' for row in catalog)+'];\n'
    (PORT/'src/catalog.rs').write_text(raw,encoding='utf8')
    (PORT/'coverage.json').write_text(json.dumps(catalog,ensure_ascii=False,indent=2),encoding='utf8')
    BUNDLE.mkdir(parents=True,exist_ok=True)
    core = (PORT/'src/rules.rs').read_text(encoding='utf8').replace('include!("catalog.rs");',raw)
    engine = (PORT/'src/engine.rs').read_text(encoding='utf8')
    if (PORT/'src/extra.rs').exists():
        engine+='\npub mod extra {\n'+(PORT/'src/extra.rs').read_text(encoding='utf8')+'\n}\n'
    if (PORT/'src/advanced.rs').exists():
        engine+='\npub mod advanced {\n'+(PORT/'src/advanced.rs').read_text(encoding='utf8')+'\n}\n'
    if (PORT/'src/table.rs').exists():
        engine+='\npub mod table {\n'+(PORT/'src/table.rs').read_text(encoding='utf8')+'\n}\n'
    if (PORT/'src/fork_features.rs').exists():
        engine+='\npub mod fork_features {\n'+(PORT/'src/fork_features.rs').read_text(encoding='utf8')+'\n}\n'
    if (PORT/'src/catastrophes.rs').exists():
        engine+='\npub mod catastrophes {\n'+(PORT/'src/catastrophes.rs').read_text(encoding='utf8')+'\n}\n'
    engine=re.sub(r'^pub fn ', 'pub(in crate::game) fn ',engine,flags=re.M)
    (BUNDLE/'runtime.rs').write_text('pub mod rules {\n'+core+'\n}\n'+engine,encoding='utf8')
    # Derive hook bindings from the pinned real source. No guessed parameter types.
    hooks = json.loads((ROOT/'horizon/native-hooks.json').read_text())
    by_name = {x['name']:x for x in hooks['hooks']}
    manifest = f'[mod]\nid = "wyr"\nversion = "0.1.0"\npriority = 100\n\n[compatibility]\nhorizon_revision = "{REV}"\n\n[[module]]\nid = "runtime"\nfile = "crates/horizon-client/src/game.rs"\nname = "wyr"\nsource = "runtime.rs"\n'
    manifest+='\n[[module]]\nid = "shared"\nfile = "crates/horizon-world/src/lib.rs"\nname = "wyr_shared"\nsource = "shared.rs"\nvisibility = "pub"\n'
    (BUNDLE/'shared.rs').write_text((PORT/'src/shared.rs').read_text(encoding='utf8'),encoding='utf8')
    specs = json.loads((PORT/'hooks.json').read_text())
    for name, spec in specs.items():
        hook = by_name.get(name,{'target':spec.get('target')})
        file = name+'.rs'
        (BUNDLE/file).write_text(spec['code']+'\n',encoding='utf8')
        manifest += f'\n[[injection]]\nid = "{name}"\nkind = "{spec.get("kind","HEAD")}"\ntarget = "{hook["target"]}"\nhelper = "{file}"\nbindings = [\n'
        for binding in spec['bindings']:
            manifest += '  { '+', '.join(k+' = '+json.dumps(v) for k,v in binding.items())+' },\n'
        manifest += ']\n'
        if 'variable' in spec:
            manifest+='variable = { '+', '.join(k+' = '+(json.dumps(v) if isinstance(v,str) else str(v)) for k,v in spec['variable'].items())+' }\n'
            manifest+='require = '+str(spec['require'])+'\nordinal = '+str(spec['ordinal'])+'\n'
        for key in ('selector','argument','require','ordinal'):
            if key in spec and 'variable' not in spec:
                manifest+=key+' = '+(json.dumps(spec[key]) if isinstance(spec[key],str) else str(spec[key]))+'\n'
    (BUNDLE/'horizon-mixin.toml').write_text(manifest,encoding='utf8')
    compat = json.loads((PORT/'compatibility.json').read_text())
    compat['horizon_revision'] = REV
    compat.pop('hooks',None)
    compat['horizon_mixins_revision']='5a64af938ec86b00d53d16a408aabc0038fb3e57'
    (PORT/'compatibility.json').write_text(json.dumps(compat,indent=2),encoding='utf8')
    print(f'Generated WYR: {len(catalog)} choices, {sum(c["supported"] for c in catalog)} enabled mechanics, {len(specs)} hooks')

if __name__ == '__main__': main()
