# Orbital Laser integration

`orbital_integration.apply(stage)` copies the complete MIT-licensed upstream
engine at commit `51468c5f69c8f61cb8982a44700853390eb23ed9`, its shaders and
its native host into the owned Horizon stage. The original sources stay intact.
It embeds the original terrain plans, readbacks, staged edits, sounds and GPU
post passes; these are not approximate replacement explosions.

The six public WYR numbers are fixed:

| Number | Original key | Event |
|---|---|---|
| 1 | blackhole | Black Hole |
| 2 | meteor | Meteor Storm |
| 3 | planetkiller | Planet Killer |
| 4 | rod | Rod from God |
| 5 | sinkhole | Sinkhole |
| 6 | payload | Payload |

The copied engine gains `wyr_force(number, position)`, which sets the original
engine's target directly and calls its original `fire`. It does not use a
crosshair raycast. Terrain is enabled, follow camera disabled. No item,
manual command, wheel or orbital screen hook is installed.

## Parent wiring

- Include `port/src/catastrophes.rs` as `wyr::catastrophes`.
- At server countdown zero call `dispatch(game, affected_uuid, number, position)`.
  Only consume the server pending job if it returns true.
- On `net_from_server` call `catastrophes::receive(game, channel, data)`.
- From mutable window frame call `catastrophes::tick(game)`.
- On world exit call `catastrophes::clear()`.
- Add `horizon_world::wyr_shared::host_uuid() -> Option<u128>`.
- Call `apply(stage)` after refreshing the generated WYR module, since the
  function appends a separate `game::orbital_glue` module.
- Merge the upstream `orbital` assets/data and sounds into combined content;
  keep the single content mod's id `wyr`. Include original MIT LICENSE/credit.
- The new path dependency changes Cargo.lock: first check must permit a lock
  update, then subsequent checks can use `--locked`.

## Authority and current limits

The original native orbital host runs playback on the rendering window, then
sends edits and commands to the integrated server, which requires operator4.
WYR sends every forced job to the integrated owner's window. The affected
survival guest is not granted operator or raw console permissions. A busy
strike keeps later accepted jobs in a FIFO queue.

Dispatch currently rejects dedicated servers without a rendering owner,
missing host or a target in a different
dimension from the host. It returns false, so the authoritative pending WYR
event can remain visible rather than silently disappearing. Full independent
server playback and cross-dimension rendering require further engine work.
After an authoritative WYR dispatch, the integrated rendering owner's work is
accepted for a bounded180 seconds of game time per queued job, including survival
without cheats. This scope is tied to the server's recorded owner UUID; arbitrary
guests remain subject to the original operator4 gate. World exit clears the grant.

## Verification completed

- Script parsed via Python `py_compile`.
- Full direct patch application succeeded on a scratch stage with original
  pinned Horizon sources, including all twelve installed injections.
- The copied real Orbital crate, generated shaders and `wyr_force` compiled
  successfully with Rust stable (`cargo check`, one upstream unused import).

The complete Horizon build and visual game session are not verified by these
checks. The upstream native bundle pins a newer Horizon revision; full compile
can reveal host helper API differences that must be adapted in this script.
