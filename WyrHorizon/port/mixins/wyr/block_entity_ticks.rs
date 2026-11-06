fn wyr_block_entity_ticks(game: &crate::game::Game<'_>, block: &str, ticks: &mut bool) { if block==crate::game::wyr::table::TABLE {*ticks=true;} }
