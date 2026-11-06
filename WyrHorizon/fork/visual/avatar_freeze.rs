
// WYR full-engine visual patch: retain each player's last posed bones.
thread_local! {
    static WYR_PLAYER_POSES:std::cell::RefCell<std::collections::BTreeMap<u32,Poses>>=
        std::cell::RefCell::new(std::collections::BTreeMap::new());
}
pub fn setup(m:&Mob,s:&State,p:&mut Poses,w_anims:&HashMap<String,super::keyframes::Animation>) {
    let player=&*m.kind.id=="minecraft:player";
    let frozen=player && if m.id==0 {
        horizon_world::wyr_shared::client_count("no_animations")>0
    }else{horizon_world::wyr_shared::entity_count(m.id as i32,"no_animations")>0};
    if frozen {
        WYR_PLAYER_POSES.with(|cache|{
            if let Some(previous)=cache.borrow().get(&m.id) {p.copy_from(previous);}
        });
        player_model_visibility(p,m.extra.hidden_model_parts,m.extra.spectator);
        return;
    }
    wyr_original_setup(m,s,p,w_anims);
    if player {
        WYR_PLAYER_POSES.with(|cache|{
            let mut cache=cache.borrow_mut();
            if cache.len()>1024 {cache.clear();}
            cache.entry(m.id).or_default().copy_from(p);
        });
    }
}
