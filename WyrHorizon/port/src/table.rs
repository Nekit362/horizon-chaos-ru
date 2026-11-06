use crate::game::Game;
use horizon_client::{containers::ModBlockEntity, items::ItemStack};
use horizon_world::{attachments::{AttachmentType,AttachmentCodec,AttachmentError,Holder},nbt::{Compound,Tag}};
use horizon_core::random::RandomSource;

pub const TABLE:&str="wyr:unrathering_table";
const QUESTS:[(&str,&str);12]=[
    ("diamond_block","алмазный блок"),("sniffer_egg","яйцо нюхача"),
    ("netherite_ingot","незеритовый слиток"),("echo_shard","осколок эха"),
    ("music_disc_pigstep","пластинка Pigstep"),("heart_of_the_sea","сердце моря"),
    ("axolotl_bucket","ведро с аксолотлем"),("ender_eye","око Эндера"),
    ("respawn_anchor","якорь возрождения"),("gravel","гравий"),
    ("cake","торт"),("end_crystal","кристалл Энда"),
];
fn number(nbt:&Compound,key:&str)->i32 {match nbt.get(&key.into()){Some(Tag::Int(v))=>*v,_=>0}}
fn set_number(nbt:&mut Compound,key:&str,value:i32) {nbt.insert(key.into(),Tag::Int(value));}
fn wanted(nbt:&Compound)->Option<String> {
    let i=number(nbt,"index").max(0) as usize;
    match nbt.get(&"quests".into()){Some(Tag::List(items))=>match items.get(i){Some(Tag::String(s))=>Some(s.to_string_lossy()),_=>None},_=>None}
}
fn encode_done(done:&bool)->Result<Compound,AttachmentError>{Ok([("done".into(),Tag::Byte(i8::from(*done)))].into())}
fn decode_done(nbt:&Compound)->Result<bool,AttachmentError>{Ok(matches!(nbt.get(&"done".into()),Some(Tag::Byte(1))))}
static COMPLETED:AttachmentType<bool>=AttachmentType{namespace:super::ID,name:"completed",default:||false,codec:Some(AttachmentCodec{encode:encode_done,decode:decode_done}),copy_on_death:false,sync:false};

pub fn world_disabled(game:&mut Game<'_>)->bool {
    if game.dim.id()=="minecraft:overworld" {
        if let Ok(done)=game.native_attachments(Holder::Level).and_then(|a|a.get(&COMPLETED)) {
            super::SERVER.with(|s|s.borrow_mut().completed=done);
        }
    }
    super::SERVER.with(|s|s.borrow().completed)
}
pub fn create(block:&str,entity:&mut Option<(String,Compound)>) {
    if block==TABLE {*entity=Some((TABLE.into(),Compound::new()));}
}
pub fn click(game:&mut Game<'_>,pos:[i32;3],canceled:&mut bool) {
    if *canceled||!game.native_on(super::ID)||game.native_block(pos)!=TABLE{return;}
    *canceled=true;
    let Some(player)=super::local(game) else{return;};
    if game.dim.id()!="minecraft:overworld" {super::tell(game,player,"Стол отмены выбора работает в обычном мире.");return;}
    let held=game.held().map(|s|s.item.to_string());
    let mut quests:Vec<&str>=QUESTS.iter().map(|(id,_)|*id).collect();
    for i in (1..quests.len()).rev() {let j=game.world.random.next_int_bound((i+1) as i32).unwrap_or(0) as usize;quests.swap(i,j);}
    let result=game.native_with_mod_block_entity(pos,|entity| {
        if number(&entity.nbt,"ending")>0 {return (false,None);}
        let first=!entity.nbt.contains_key(&"quests".into());
        if first {
            let items=quests.iter().take(5).map(|id|Tag::String(format!("minecraft:{id}").as_str().into())).collect();
            entity.nbt.insert("quests".into(),Tag::List(items));set_number(&mut entity.nbt,"index",0);
        }
        let consumed=!first&&wanted(&entity.nbt).as_deref()==held.as_deref();
        if consumed {let next=number(&entity.nbt,"index")+1;set_number(&mut entity.nbt,"index",next);}
        let next=wanted(&entity.nbt);
        if next.is_none() {set_number(&mut entity.nbt,"ending",1);}
        (consumed,next)
    });
    if let Some((consumed,next))=result {
        if consumed {game.consume_held();}
        if let Some(next)=next {
            let ru=QUESTS.iter().find(|(id,_)|next.strip_prefix("minecraft:")==Some(*id)).map_or(next.as_str(),|(_,ru)|*ru);
            super::tell(game,player,&format!("Следующее подношение: {ru}, 1 шт. Возьмите предмет в руку и нажмите на стол."));
        } else {super::tell(game,player,"Все пять подношений приняты. Предложения выбора отключаются…");}
        game.native_level_play_sound("minecraft:block.enchantment_table.use",pos.map(|v|f64::from(v)+0.5),4,1.0,2.0);
        game.native_block_entity_sync(pos);
    }
}
pub fn tick(game:&mut Game<'_>,entity:&mut ModBlockEntity) {
    if entity.id!=TABLE||!game.native_on(super::ID){return;}
    let ending=number(&entity.nbt,"ending");
    if ending==0||ending>200{return;}
    if ending==1 {
        if let Ok(a)=game.native_attachments(Holder::Level){let _=a.set(&COMPLETED,true);}
        super::SERVER.with(|s|s.borrow_mut().completed=true);
    }
    if ending==1||ending==20||ending==120||ending==200 {
        for player in game.native_server_players() {
            let Ok(mut p)=game.native_attachments(Holder::Player(player.uuid)).and_then(|a|a.get(&super::PLAYER_STATE)) else{continue;};
            if ending==1 {p.rate=0;p.countdown=0;p.pending=None;}
            if ending==20 {super::tell(game,player.uuid,"Предложения «Что бы ты выбрал?» отключены!");}
            if ending==120 {super::tell(game,player.uuid,"…");}
            if ending==200 {
                p.nonce=p.nonce.wrapping_add(1);
                p.pending=Some([
                    super::rules::Pair{positive:"unlimited_bacon".into(),negative:"no_more_video_games".into()},
                    super::rules::Pair{positive:"unlimited_games".into(),negative:"no_games".into()},
                ]);
                super::tell(game,player.uuid,"Последняя дилемма. Как и в оригинале, отрицательный выбор блокирует игрока в этом мире.");
            }
            if let Ok(a)=game.native_attachments(Holder::Player(player.uuid)){let _=a.set(&super::PLAYER_STATE,p.clone());}
            super::SERVER.with(|s|s.borrow_mut().players.insert(player.uuid,p.clone()));super::sync(game,player.uuid,&p);
        }
    }
    set_number(&mut entity.nbt,"ending",ending+1);
}
pub fn render(game:&Game<'_>,entity:&ModBlockEntity,draw:&mut crate::game::native_block_entity_draw::BlockEntityDraw) {
    if entity.id!=TABLE {return;}
    if let Some(item)=wanted(&entity.nbt) {
        draw.push();draw.translate([0.5,1.0,0.5]);
        draw.rotate_about([0.0,1.0,0.0],(draw.game_time as f32+draw.partial)*3.0,[0.0;3]);draw.scale([0.5;3]);
        game.native_draw_item(draw,&ItemStack::new(&item,1),horizon_assets::item_model::DisplayContext::Ground,None,0);
        draw.pop();
    }
}
