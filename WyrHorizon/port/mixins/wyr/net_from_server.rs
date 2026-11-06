fn wyr_net_from_server(game: &mut crate::game::Game<'_>, channel: &str, data: &[u8]) { crate::game::wyr::receive_client(game,channel,data); }
