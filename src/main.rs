mod buddy;
mod weapons;
mod display;
mod narration;
mod game_state;


use buddy::Buddy;
use weapons::{Weapon, WeaponManager};
use display::render_ui;
use narration::{generate_damage_reaction, generate_weapon_use_narration};
use crate::narration::generate_typing_effect;
use std::{thread, time};

fn main() {
    println!("Welcome to Text Buddy!");

    // Ask the player to input a name for the Buddy
    println!("Please enter a name for your buddy:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string(); // Remove newline characters

    // Load game state if it exists, else create new game state
    let mut game_state = game_state::load_game();
    let mut buddy = Buddy::new(&name, 100); // Start with full health
    let mut weapon_manager = WeaponManager::new();

    // Unlock weapons based on points
    weapon_manager.unlock_weapons(game_state.points);

    loop {
        // Display current points and buddy health
        println!("\nPoints: {}", buddy.points);
        println!("Health: {}", buddy.health);
        render_ui(&buddy, &weapon_manager);

        println!("Choose an action: \n1. Use Weapon \n2. Check Buddy \n3. Wait \n4. Revive Buddy \n5. Save");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                // Choose a weapon first
                println!("Choose a weapon from the following list of unlocked weapons:");

                weapon_manager.unlock_weapons(buddy.points); // Unlock weapons based on points
                weapon_manager.display_available_weapons(); // Display unlocked weapons

                let mut weapon_choice = String::new();
                std::io::stdin().read_line(&mut weapon_choice).unwrap();

                if let Ok(choice) = weapon_choice.trim().parse::<usize>() {
                    if let Some(weapon) = weapon_manager.choose_weapon_by_index(choice) {
                        // Calculate damage and apply it
                        let damage = weapon.use_weapon();
                        buddy.take_damage(damage);

                        // Add points after using the weapon
                        buddy.gain_points(damage);

                        // Handle specific behavior for Gravity Gun
                        if weapon.name == "Gravity Gun" {
                            weapon.use_gravity_gun(&buddy.name);
                        } else {
                            // Generate weapon use narration with gradual text display
                            generate_typing_effect(&generate_weapon_use_narration(&weapon.name, &buddy.name));
                        }

                        // Generate buddy's damage reaction with gradual text display
                        generate_typing_effect(&generate_damage_reaction(&buddy.name, ""));
                    } else {
                        println!("Invalid weapon choice.");
                    }
                }
            }
            "2" => {
                generate_typing_effect(&format!("{} looks at you with a puzzled expression.", buddy.name));
            }
            "3" => {
                buddy.random_idle();
                thread::sleep(time::Duration::from_secs(5));
            }
            "4" => {
                if buddy.health <= 0 {
                    buddy.revive(); // Revive the buddy
                    println!("{} has been revived!", buddy.name);
                } else {
                    println!("{} is still alive!", buddy.name);
                }
            }
            "5" => {
                // Save game state
                game_state.points = buddy.points; // Save the points to game state
                game_state::save_game(&game_state);
                println!("Game saved.");
            }
            _ => println!("Invalid choice!"),
        }

        if buddy.health <= 0 {
            println!("{} has been defeated! Game over.", buddy.name);
            break;
        }
    }
}
