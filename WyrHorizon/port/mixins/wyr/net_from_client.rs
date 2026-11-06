fn wyr_net_from_client(game: &mut crate::game::Game<'_>, player: u128, channel: &str, data: &[u8]) { crate::game::wyr::receive_server(game,player,channel,data); }
