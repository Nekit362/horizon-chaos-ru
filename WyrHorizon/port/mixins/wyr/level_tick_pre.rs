fn wyr_level_tick_pre(game: &mut crate::game::Game<'_>, dimension: &str) { crate::game::wyr::advanced::level_pre(game,dimension); crate::game::wyr::fork_features::prepare_shovel(game); }
