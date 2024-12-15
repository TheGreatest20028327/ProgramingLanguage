use crate::buddy::Buddy;
use crate::weapons::WeaponManager;

pub fn render_ui(buddy: &Buddy, weapon_manager: &WeaponManager) {
    println!("\n--- Buddy States ---");
    println!("Buddy: {} | Health: {}", buddy.name, buddy.health);
    println!("Weapons: {}", weapon_manager.weapons.len());
    println!("--- ha this dude is in trouble ---");
}