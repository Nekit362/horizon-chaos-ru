fn wyr_living_incoming_damage(entity: i32, kind: &str, damage_type: &str, amount: &mut f32, canceled: &mut bool) { crate::game::wyr::damage(entity,kind,damage_type,amount,canceled); }
