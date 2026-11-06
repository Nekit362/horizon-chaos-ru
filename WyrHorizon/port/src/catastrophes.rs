//! Fixed WYR catastrophe numbers. Countdown and drawing belong to WYR's offer UI.
use crate::game::Game;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::sync::Mutex;
static AUTHORIZED: Mutex<BTreeMap<u128, i64>> = Mutex::new(BTreeMap::new());
static HOST_CONTEXT: Mutex<Option<(u128, String)>> = Mutex::new(None);
thread_local! { static WAITING: RefCell<VecDeque<(u8,[f64;3])>> = RefCell::new(VecDeque::new()); }
pub const CHANNEL: &str = "wyr:catastrophe";
pub const NAMES: [&str; 6] = ["Чёрная дыра", "Метеоритный шторм", "Убийца планет", "Жезл Бога", "Провал грунта", "Боевой заряд"];

/// Runs at ServerTickEnd, outside as_guest. The integrated owner is also a
/// guest internally, so `lan_acting.is_none()` cannot identify their UUID.
pub(in crate::game) fn refresh_host(game:&Game<'_>) {
    let owner=game.lan.as_ref().and_then(|h|h.hub.singleplayer_owner);
    horizon_world::wyr_shared::set_host_uuid(owner);
    let context=owner.and_then(|uuid|game.native_server_players().into_iter().find(|p|p.uuid==uuid).map(|p|(uuid,p.dimension)));
    *HOST_CONTEXT.lock().unwrap_or_else(|e|e.into_inner())=context;
}

/// Called by the authoritative WYR countdown for this player. Coordinates are
/// captured at zero, rather than when the warning first appeared.
pub(in crate::game) fn dispatch(game: &mut Game<'_>, player: u128, number: u8, at: [f64; 3]) -> bool {
    if !(1..=6).contains(&number) || at.iter().any(|v| !v.is_finite()) { return false; }
    if game.lan.is_some() {refresh_host(game);}
    let Some((host,dimension)) = HOST_CONTEXT.lock().unwrap_or_else(|e|e.into_inner()).clone() else { return false; };
    // The host's native engine performs the server-authorized event. A guest
    // never receives a console permission grant. Cross-dimension rendering and
    // dedicated servers need a separate server playback host.
    // Actor hooks borrow the hub out of game.lan. Their live player snapshot
    // remains available even though native_server_players is then empty.
    let source=game.native_acting_player().filter(|p|p.uuid==player)
        .or_else(||game.native_server_players().into_iter().find(|p|p.uuid==player));
    let Some(source)=source else{return false;};
    if dimension!=source.dimension {return false;}
    let mut data = vec![number];
    for v in at { data.extend_from_slice(&v.to_le_bytes()); }
    if game.native_net_send_to_player(host, CHANNEL, &data).is_err() {return false;}
    // Only the integrated owner's native renderer may execute an internally
    // dispatched WYR job; this never authorizes another player's messages.
    let mut active=AUTHORIZED.lock().unwrap_or_else(|e|e.into_inner());
    let now=game.world.game_time;
    let previous=active.get(&host).copied().unwrap_or(now).max(now);
    active.insert(host,previous.saturating_add(180*20));
    true
}

/// Only server-to-client traffic reaches this function; never accept a client's
/// proposed catastrophe or position as an authoritative WYR event.
pub(in crate::game) fn receive(_game: &Game<'_>, channel: &str, data: &[u8]) {
    if channel != CHANNEL || data.len() != 25 { return; }
    let at = std::array::from_fn(|i| f64::from_le_bytes(data[1+i*8..9+i*8].try_into().unwrap()));
    if !(1..=6).contains(&data[0]) || at.iter().any(|v|!v.is_finite()){return;}
    WAITING.with(|q|q.borrow_mut().push_back((data[0],at)));
}

/// Called from the mutable client frame. Preserve queued disasters while a
/// previous strike is still running; there is no second wheel or choice.
pub(in crate::game) fn tick(game: &mut Game<'_>) {
    if !game.native_window() || !game.native_on(super::ID) {return;}
    let job=WAITING.with(|q|q.borrow().front().copied());
    if let Some((number,at))=job {
        if start(game,number,at) {WAITING.with(|q|{q.borrow_mut().pop_front();});}
    }
}
pub(in crate::game) fn clear() {
    WAITING.with(|q|q.borrow_mut().clear());
    AUTHORIZED.lock().unwrap_or_else(|e|e.into_inner()).clear();
    *HOST_CONTEXT.lock().unwrap_or_else(|e|e.into_inner())=None;
}
pub(in crate::game) fn authorized_owner(game:&Game<'_>,player:u128)->bool {
    let owner=game.lan.as_ref().and_then(|h|h.hub.singleplayer_owner)
        .or_else(||HOST_CONTEXT.lock().unwrap_or_else(|e|e.into_inner()).as_ref().map(|c|c.0));
    if owner!=Some(player){return false;}
    AUTHORIZED.lock().unwrap_or_else(|e|e.into_inner()).get(&player).is_some_and(|until|game.world.game_time<*until)
}

pub(in crate::game) fn start(game: &mut Game<'_>, number: u8, at: [f64; 3]) -> bool {
    crate::game::orbital_glue::wyr_start(game, number, at)
}
