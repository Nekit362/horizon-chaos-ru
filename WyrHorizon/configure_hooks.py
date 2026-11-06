import json,pathlib
ROOT=pathlib.Path(__file__).resolve().parent
specs={}
def add(name,params,call,mutable=True,game=True):
    bindings=[];args=[]
    if game:
        ty="&mut crate::game::Game<'_>" if mutable else "&crate::game::Game<'_>"
        bindings.append(dict(name='game',ty=ty,value='&mut *self' if mutable else '&*self'))
        args.append('game: '+ty)
    for name_,ty,value in params:
        bindings.append(dict(name=name_,ty=ty,value=value));args.append(name_+': '+ty)
    specs[name]=dict(bindings=bindings,code='fn wyr_'+name+'('+', '.join(args)+') { '+call+' }')
def p(name,ty,value=None):return (name,ty,value or name)
M='crate::game::wyr::'
add('player_tick',[],M+'tick_player(game);')
add('server_tick',[p('phase','horizon_client::mod_api::ServerTickPhase')],M+'server_tick(game,phase);')
add('world_saved',[],M+'save(game);')
add('server_stopping',[],M+'save(game);')
add('net_from_client',[p('player','u128'),p('channel','&str'),p('data','&[u8]')],M+'receive_server(game,player,channel,data);')
add('net_from_server',[p('channel','&str'),p('data','&[u8]')],M+'receive_client(game,channel,data);')
add('client_disconnecting',[],M+'reset_client();',game=False)
add('frame',[],M+'frame(game);')
add('key',[p('key','&str'),p('pressed','bool')],M+'key(game,key,pressed);')
add('command',[p('command','&str'),p('ran','&mut bool','&mut *ran')],M+'command(game,command,ran);')
add('mods',[p('mods','&mut Vec<horizon_client::mod_api::NativeMod>','&mut *mods')],M+'listed(mods);')
add('gui_render_post',[p('batch','&mut horizon_client::hud::HudBatch','&mut *batch')],M+'draw(game,batch);',False)
add('hud',[p('batch','&mut horizon_client::hud::HudBatch','&mut *batch')],M+'hud(game,batch);')
add('screen_mouse_button_pressed_pre',[p('button','i32'),p('canceled','&mut bool','&mut *canceled')],M+'mouse(game,button,canceled);',False)
add('movement_input',[p('forward','&mut f32','&mut *forward'),p('strafe','&mut f32','&mut *strafe')],M+'movement(forward,strafe);',False)
add('compute_fov',[p('degrees','&mut f32','&mut *degrees')],M+'fov(game,degrees);',False)
add('health_hud',[p('inputs','&horizon_client::hud_layout::HealthInputs'),p('wi','i32'),p('hi','i32'),p('draws','&mut Option<Vec<horizon_client::hud_layout::GuiDraw>>','&mut *draws')],M+'health_hud(game,inputs,wi,hi,draws);')
add('gui_layer_pre',[p('layer','&str'),p('batch','&mut horizon_client::hud::HudBatch','&mut *batch')],M+'layer_pre(layer,batch);')
add('gui_layer_post',[p('layer','&str'),p('batch','&mut horizon_client::hud::HudBatch','&mut *batch')],M+'layer_post(layer,batch);')
add('living_incoming_damage',[p('entity','i32'),p('kind','&str'),p('damage_type','&str'),p('amount','&mut f32','&mut *amount'),p('canceled','&mut bool','&mut *canceled')],M+'damage(entity,kind,damage_type,amount,canceled);',game=False)
add('living_knockback_v2',[p('attacker','i32'),p('strength','&mut f64','&mut *strength')],M+'knockback(attacker,strength);',game=False)
add('right_click_block',[p('pos','[i32; 3]'),p('face','[i32; 3]'),p('canceled','&mut bool','&mut *canceled')],M+'right_click(game,pos,face,canceled);')
add('place_block',[],M+'placed(game);')
add('can_eat',[p('stack','&horizon_client::items::ItemStack'),p('can_eat','&mut bool','&mut *can_eat')],M+'can_eat(game,stack,can_eat);',False)
add('living_entity_use_item_finish',[p('stack','&horizon_client::items::ItemStack'),p('result','&mut Option<horizon_client::items::ItemStack>','&mut *result')],M+'consumed(game,stack,result);')
add('player_jump',[],M+'jumped(game);')
add('loot_drops',[p('context',"&horizon_client::mod_api::NativeLootDrops<'_>"),p('drops','&mut Vec<horizon_client::items::ItemStack>','&mut *drops')],M+'loot(game,context,drops);')
add('player_respawned',[p('player','u128')],M+'respawn(game,player);')
add('item_damaged',[p('amount','&mut i32','&mut *amount'),p('cancel','&mut bool','&mut *cancel')],M+'item_damage(game,amount,cancel);')
add('modify_default_components',[p('item','&str'),p('components','&mut serde_json::Map<String, serde_json::Value>','&mut *components')],
    'if item=="minecraft:leather" { components.insert("minecraft:food".into(),serde_json::json!({"nutrition":2,"saturation":0.1})); components.insert("minecraft:consumable".into(),serde_json::json!({"consume_seconds":1.6,"animation":"eat","sound":"minecraft:entity.generic.eat"})); }',game=False)
specs['shield_angle']={
    'target':"horizon_client::game::shield::Game<'_>::player_blocking",'kind':'MODIFY_VARIABLE',
    'variable':{'name':'angle','ordinal':0,'at':'STORE'},'require':1,'ordinal':0,
    'bindings':[{'name':'game','ty':"&crate::game::Game<'_>",'value':'&*self'},{'name':'value','ty':'f64','value':'$value'}],
    'code':"fn wyr_shield_angle(game:&crate::game::Game<'_>,value:f64)->f64 { crate::game::wyr::shield_angle(game,value) }"
}
extra_file=ROOT/'port/extra-hooks.json'
if extra_file.exists():
    extra=json.loads(extra_file.read_text(encoding='utf8'))
    for name,callback in extra['callbacks'].items():
        bindings=callback['bindings']
        if name in specs:
            existing={b['name'] for b in specs[name]['bindings']}
            for b in bindings:
                if b['name'] not in existing:specs[name]['bindings'].append(b)
            # Existing callbacks currently already bind all shared parameters.
            names={b['name'] for b in bindings}
            missing=names-existing
            if missing:
                spec=specs[name]
                signature=spec['code'][:spec['code'].index(') {')]
                for b in bindings:
                    if b['name'] in missing:signature+=', '+b['name']+': '+b['ty']
                spec['code']=signature+spec['code'][spec['code'].index(') {'):]
            code=specs[name]['code'];last=code.rfind('}')
            specs[name]['code']=code[:last]+callback['call']+' '+code[last:]
        else:
            args=', '.join(b['name']+': '+b['ty'] for b in bindings)
            specs[name]={'bindings':bindings,'code':'fn wyr_'+name+'('+args+') { '+callback['call']+' }'}
def custom(name,target,kind,bindings,code,**options):
    specs[name]=dict(target=target,kind=kind,bindings=[dict(name=n,ty=t,value=v) for n,t,v in bindings],code=code,**options)
G=('game',"&mut crate::game::Game<'_>",'&mut *self')
R=('game',"&crate::game::Game<'_>",'&*self')
A='crate::game::wyr::advanced::'
spec=specs['interact_entity'];at=spec['code'].index('{')+1
spec['code']=spec['code'][:at]+A+'enter_action(game); '+spec['code'][at:]
at=spec['code'].rfind('}');spec['code']=spec['code'][:at]+A+'tame_interact(game,entity,stack,claimed); '+spec['code'][at:]
add('use_item',[],A+'enter_action(game);')
custom('fishing_treasure','horizon_client::game::fishing::fishing_loot','MODIFY_VARIABLE',
       [('value',"&'static str",'$value')],"fn wyr_treasure(value:&'static str)->&'static str { "+A+'fishing_table(value) }',
       variable={'name':'table','ordinal':0,'at':'STORE'},require=1,ordinal=0)
spec=specs['player_tick'];at=spec['code'].rfind('}');spec['code']=spec['code'][:at]+A+'mounted_tame(game); '+spec['code'][at:]
add('level_tick_pre',[p('dimension','&str')],A+'level_pre(game,dimension);')
add('loot_table_load',[p('id','&str'),p('table','&mut serde_json::Value','&mut *table')],
    'if id=="minecraft:gameplay/piglin_bartering" { if table.get("functions").is_none() {table["functions"]=serde_json::json!([]);} if let Some(f)=table["functions"].as_array_mut() {f.push(serde_json::json!({"function":"wyr:barter"}));} }',game=False)
add('loot_function',[p('kind','&str'),p('ctx',"&crate::loot::NativeLootContext<'_>"),p('stack','&mut crate::loot::LootStack','&mut *stack')],
    'if kind=="wyr:barter" && crate::wyr_shared::barter() {use horizon_core::random::RandomSource;let mut r=horizon_core::random::LegacyRandom::new(ctx.seed);*stack=crate::loot::LootStack::new("minecraft:ender_eye");stack.count=1+r.next_int_bound(4).unwrap_or(0);}',game=False)
custom('cheap_trade',"horizon_client::game::trading::Game<'_>::open_merchant_state",'OVERWRITE',
       [G,('state','crate::game::trading::MerchantState','state')],
       """fn wyr_trade(game:&mut crate::game::Game<'_>,mut state:crate::game::trading::MerchantState) {
           if state.session.villager && crate::game::wyr::has(game,"cheap_trades") {
               for offer in &mut state.session.offers {offer.special_price_diff=-(offer.base_cost_a.stack.count as i32);}
           }
           let mut s=crate::game::stations::Station::new_kind(crate::game::stations::Kind::Merchant,[0,0,0]);
           s.merchant=Some(Box::new(state));game.station=Some(s);game.screen=crate::game::Screen::Inventory;game.capture(false);game.right=false;
       }""")
add('hears_projectile_impacts',[p('hears','&mut bool','&mut *hears')],'*hears=true;',mutable=False)
add('projectile_impact',[p('dimension','&str'),p('kind','&str'),p('owner','u32'),p('at','[f64;3]'),p('canceled','&mut bool','&mut *canceled')],A+'egg_impact(game,dimension,kind,owner,at,*canceled);')
T='crate::game::wyr::table::'
add('block_entity',[p('block','&str'),p('entity','&mut Option<(String, horizon_world::nbt::Compound)>','&mut *entity')],T+'create(block,entity);')
add('block_entity_ticks',[p('block','&str'),p('ticks','&mut bool','&mut *ticks')],'if block==crate::game::wyr::table::TABLE {*ticks=true;}',mutable=False)
add('block_entity_tick',[p('entity','&mut horizon_client::containers::ModBlockEntity','&mut *entity')],T+'tick(game,entity);')
add('block_entity_update_tag',[p('entity','&horizon_client::containers::ModBlockEntity'),p('tag','&mut Option<horizon_world::nbt::Compound>','&mut *tag')],
    'if entity.id==crate::game::wyr::table::TABLE {*tag=Some(entity.nbt.clone());}',game=False)
add('block_entity_submit',[p('entity','&horizon_client::containers::ModBlockEntity'),p('draw','&mut crate::game::native_block_entity_draw::BlockEntityDraw','&mut *draw')],T+'render(game,entity,draw);',mutable=False)
spec=specs['right_click_block'];at=spec['code'].rfind('}');spec['code']=spec['code'][:at]+T+'click(game,pos,canceled); '+spec['code'][at:]
spec=specs['player_tick'];at=spec['code'].rfind('}');spec['code']=spec['code'][:at]+'crate::game::wyr::fork_features::tick(game); '+spec['code'][at:]
spec=specs['level_tick_pre'];at=spec['code'].rfind('}');spec['code']=spec['code'][:at]+'crate::game::wyr::fork_features::prepare_shovel(game); '+spec['code'][at:]
spec=specs['frame'];at=spec['code'].rfind('}');spec['code']=spec['code'][:at]+'if game.player.is_some(){crate::game::wyr::fork_features::prepare_shovel(game);} '+spec['code'][at:]
(ROOT/'port/hooks.json').write_text(json.dumps(specs,indent=2),encoding='utf8')
print(len(specs),'hooks configured')
