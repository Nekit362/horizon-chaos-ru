from pathlib import Path
p=Path(__file__).resolve().parent/'port/src/engine.rs'
s=p.read_text(encoding='utf8')
s=s.replace('game.native_tell(','tell(game,').replace('game.native_give_stack(stack)','game.native_give_stack(id,stack)')
a=s.index('fn load(game:'); b=s.index('pub fn server_tick',a)
s=s[:a]+'''fn load(game: &Game<'_>) {
    let Some(root)=game.native_world_dir() else {return;};
    SERVER.with(|s|{let mut s=s.borrow_mut();if s.root!=root{*s=Server::default();s.root=root;}});
}
pub fn save(_game:&Game<'_>) {}
fn tell(game:&mut Game<'_>,id:u128,text:&str) {let _=game.native_tell(id,horizon_client::text::Text::literal(text));}
fn encode_player(p:&Player)->Result<horizon_world::nbt::Compound,horizon_world::attachments::AttachmentError> {
    let text=serde_json::to_string(p).map_err(|_|horizon_world::attachments::AttachmentError::Decode)?;
    Ok([("json".into(),horizon_world::nbt::Tag::String(text.into()))].into())
}
fn decode_player(c:&horizon_world::nbt::Compound)->Result<Player,horizon_world::attachments::AttachmentError> {
    let Some(horizon_world::nbt::Tag::String(text))=c.get(&"json".into()) else{return Err(horizon_world::attachments::AttachmentError::Decode);};
    serde_json::from_str(&text.to_string_lossy()).map_err(|_|horizon_world::attachments::AttachmentError::Decode)
}
static PLAYER_STATE:horizon_world::attachments::AttachmentType<Player>=horizon_world::attachments::AttachmentType {
    namespace:ID,name:"player",default:Player::default,
    codec:Some(horizon_world::attachments::AttachmentCodec{encode:encode_player,decode:decode_player}),
    copy_on_death:true,sync:false,
};
''' +s[b:]
a=s.index('    let mut p=SERVER.with'); b=s.index('    let request=',a)
s=s[:a]+'''    let Ok(mut p)=game.native_attachments(horizon_world::attachments::Holder::Player(id)).and_then(|a|a.get(&PLAYER_STATE)) else{return;};
    if p.nonce==0 && p.active.is_empty() {p.random=seed(id);}
    SERVER.with(|s| {let mut s=s.borrow_mut();s.entities.insert(who.entity_id,id);s.players.insert(id,p.clone());});
''' +s[b:]
s=s.replace('    if changed || SERVER.with','''    if let Ok(a)=game.native_attachments(horizon_world::attachments::Holder::Player(id)) {let _=a.set(&PLAYER_STATE,p.clone());}
    if changed || SERVER.with''')
s=s.replace('for slot in 0..=40 {','for slot in (0..36).chain(100..=103).chain(std::iter::once(-106)) {')
p.write_text(s,encoding='utf8')
