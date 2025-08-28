fn main() {
    let hero_level = 5; 
    let is_strong_enough = can_lift_hammer(hero_level);
    if is_strong_enough {
        println!("The Weapon Master says: 'You are strong enough! Take the hammer.'");
    } else {
        println!("The Weapon Master says: 'You are not yet worthy. Come back when you are stronger.'");
    }
}

fn can_lift_hammer(level: u32) -> bool {
    level > 10
}