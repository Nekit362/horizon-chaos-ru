fn wyr_trade(game:&mut crate::game::Game<'_>,mut state:crate::game::trading::MerchantState) {
           if state.session.villager && crate::game::wyr::has(game,"cheap_trades") {
               for offer in &mut state.session.offers {offer.special_price_diff=-(offer.base_cost_a.stack.count as i32);}
           }
           let mut s=crate::game::stations::Station::new_kind(crate::game::stations::Kind::Merchant,[0,0,0]);
           s.merchant=Some(Box::new(state));game.station=Some(s);game.screen=crate::game::Screen::Inventory;game.capture(false);game.right=false;
       }
