fn wyr_barter(game:&mut crate::game::Game<'_>,i:usize,items:Vec<horizon_client::items::ItemStack>) { let items=crate::game::wyr::advanced::barter(game,game.mobs[i].id,items); 
        let m = &game.mobs[i];
        let from = [m.position.x, m.position.y + 1.3, m.position.z];
        // BehaviorUtils.throwItem toward the nearest visible player
        // (NEAREST_VISIBLE_PLAYER: within 16, alive, no spectators; the
        // guests count as much as the host), else nearby.
        let at = m.position;
        let host = game
            .player
            .as_ref()
            .filter(|p| !p.health.dead_or_dying() && !game.spectator())
            .map(|p| [p.position.x, p.position.y, p.position.z]);
        let guests = game.lan.iter().flat_map(|h| {
            h.hub
                .guests
                .iter()
                .filter(|g| h.hub.here(g) && g.loaded && !g.dead && !g.spectator())
                .map(|g| g.position)
        });
        let d = |p: &[f64; 3]| at.distance_squared(Vec3::new(p[0], p[1], p[2]));
        let target = host
            .into_iter()
            .chain(guests)
            .filter(|p| d(p) < 256.0)
            .min_by(|a, b| d(a).total_cmp(&d(b)))
            .map(|p| [p[0], p[1] + 1.0, p[2]]);
        for stack in items {
            let to = target.unwrap_or_else(|| {
                let r = &mut game.world.random;
                [
                    from[0] + f64::from(r.next_float() * 8.0 - 4.0),
                    from[1],
                    from[2] + f64::from(r.next_float() * 8.0 - 4.0),
                ]
            });
            game.piglin_throw(from, to, stack);
        } }
