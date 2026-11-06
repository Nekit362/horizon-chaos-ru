fn wyr_frame(game: &mut crate::game::Game<'_>) { crate::game::wyr::frame(game); if game.player.is_some(){crate::game::wyr::fork_features::prepare_shovel(game);} }
