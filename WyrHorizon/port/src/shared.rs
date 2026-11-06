// Per-server-thread bridge for loot hooks, shared with the game crate.
use std::cell::Cell;
use std::{collections::BTreeMap,sync::{Mutex,OnceLock}};
type Flags=BTreeMap<String,u32>;
#[derive(Default)]
struct State { client:Flags, players:BTreeMap<u128,Flags>, entities:BTreeMap<i32,u128>, guests:BTreeMap<u32,u128>,host:Option<u128>, vehicles:BTreeMap<u32,(u128,Flags)>,shovel_registry:usize,shovel:std::collections::BTreeSet<u32> }
fn state()->&'static Mutex<State> {static STATE:OnceLock<Mutex<State>>=OnceLock::new();STATE.get_or_init(||Mutex::new(State::default()))}
thread_local! {static ACTOR:Cell<Option<u128>>=const{Cell::new(None)};static CLIENT_THREAD:Cell<bool>=const{Cell::new(false)};}
pub fn set_client(flags:Flags) {CLIENT_THREAD.with(|c|c.set(true));if let Ok(mut s)=state().lock(){s.client=flags;}}
pub fn client_count(id:&str)->u32 {state().lock().ok().and_then(|s|s.client.get(id).copied()).unwrap_or(0)}
pub fn entity_count(entity:i32,id:&str)->u32 {state().lock().ok().and_then(|s|s.entities.get(&entity).and_then(|uuid|s.players.get(uuid)).and_then(|f|f.get(id)).copied()).unwrap_or(0)}
pub fn player_count(uuid:u128,id:&str)->u32 {state().lock().ok().and_then(|s|s.players.get(&uuid).and_then(|f|f.get(id)).copied()).unwrap_or(0)}
pub fn guest_count(conn:u32,id:&str)->u32 {state().lock().ok().and_then(|s|s.guests.get(&conn).and_then(|uuid|s.players.get(uuid)).and_then(|f|f.get(id)).copied()).unwrap_or(0)}
pub fn host_count(id:&str)->u32 {state().lock().ok().and_then(|s|s.host.and_then(|uuid|s.players.get(&uuid)).and_then(|f|f.get(id)).copied()).unwrap_or(0)}
pub fn host_uuid()->Option<u128> {state().lock().ok().and_then(|s|s.host)}
pub fn set_host_uuid(uuid:Option<u128>) {if let Ok(mut s)=state().lock(){s.host=uuid;}}
pub fn guest_for(uuid:u128)->Option<u32> {state().lock().ok().and_then(|s|s.guests.iter().find_map(|(conn,id)|(*id==uuid).then_some(*conn)))}
pub fn entity_for(uuid:u128)->Option<i32> {state().lock().ok().and_then(|s|s.entities.iter().find_map(|(entity,id)|(*id==uuid).then_some(*entity)))}
pub fn update_player(uuid:u128,entity:i32,guest:Option<u32>,flags:Flags) {if let Ok(mut s)=state().lock(){s.entities.insert(entity,uuid);if let Some(conn)=guest{s.guests.insert(conn,uuid);}else{s.host=Some(uuid);}s.players.insert(uuid,flags);}}
pub fn update_remote(uuid:u128,entity:i32,flags:Flags) {if let Ok(mut s)=state().lock(){s.entities.insert(entity,uuid);s.players.insert(uuid,flags);}}
pub fn update_vehicle(uuid:u128,vehicle:Option<u32>,flags:Flags) {if let Ok(mut s)=state().lock(){s.vehicles.retain(|_,(owner,_)|*owner!=uuid);if let Some(vehicle)=vehicle{s.vehicles.insert(vehicle,(uuid,flags));}}}
pub fn vehicle_count(vehicle:u32,id:&str)->u32 {state().lock().ok().and_then(|s|s.vehicles.get(&vehicle).and_then(|(_,f)|f.get(id)).copied()).unwrap_or(0)}
pub struct ActorGuard(Option<u128>);
impl Drop for ActorGuard {fn drop(&mut self){ACTOR.with(|a|a.set(self.0));}}
pub fn enter_actor(uuid:Option<u128>)->ActorGuard {ActorGuard(ACTOR.with(|a|a.replace(uuid)))}
pub fn current_count(id:&str)->u32 {if let Some(uuid)=ACTOR.with(Cell::get){player_count(uuid,id)}else if CLIENT_THREAD.with(Cell::get){client_count(id)}else{0}}
pub fn shovel_ready(registry:usize)->bool{state().lock().is_ok_and(|s|s.shovel_registry==registry)}
pub fn set_shovel(registry:usize,ids:Vec<u32>){if let Ok(mut s)=state().lock(){s.shovel_registry=registry;s.shovel=ids.into_iter().collect();}}
pub fn shovel(id:u32)->bool{state().lock().is_ok_and(|s|s.shovel.contains(&id))}
thread_local! { static BARTER: Cell<bool> = const { Cell::new(false) }; }
pub fn set_barter(enabled: bool) { BARTER.with(|b| b.set(enabled)); }
pub fn barter() -> bool { BARTER.with(Cell::get) }
