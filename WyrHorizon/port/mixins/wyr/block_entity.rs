fn wyr_block_entity(game: &mut crate::game::Game<'_>, block: &str, entity: &mut Option<(String, horizon_world::nbt::Compound)>) { crate::game::wyr::table::create(block,entity); }
