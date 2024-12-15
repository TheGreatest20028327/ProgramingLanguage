pub struct Weapon {
    pub name: String,
    pub damage: u32,
    pub unlocked: bool,
    pub unlock_threshold: u32,  // Threshold to unlock
}

impl Weapon {
    pub fn new(name: &str, damage: u32, unlocked: bool, unlock_threshold: u32) -> Self {
        Weapon {
            name: name.to_string(),
            damage,
            unlocked,
            unlock_threshold,
        }
    }

    pub fn use_weapon(&self) -> u32 {
        self.damage
    }

    // Custom feature for Gravity Gun to perform actions
    pub fn use_gravity_gun(&self, buddy_name: &str) {
        println!("You aim the {} at your buddy: {}", self.name, buddy_name);
        println!("Choose an action for your buddy:");
        println!("1. Shake");
        println!("2. Throw");
        println!("3. Launch");

        let mut action_input = String::new();
        std::io::stdin().read_line(&mut action_input).unwrap();
        match action_input.trim() {
            "1" => self.shake(buddy_name),
            "2" => self.throw(buddy_name),
            "3" => self.launch(buddy_name),
            _ => println!("Invalid action!"),
        }
    }

    fn shake(&self, buddy_name: &str) {
        println!("You shake {} around like a ragdoll! {} yells: 'Wheeeeee!'", buddy_name, buddy_name);
    }

    fn throw(&self, buddy_name: &str) {
        println!("You throw {} across the room! {} screams: 'Aaaaah!'", buddy_name, buddy_name);
    }

    fn launch(&self, buddy_name: &str) {
        println!("You launch {} into the air! {} shouts: 'I'm flying!'", buddy_name, buddy_name);
    }
}

pub struct WeaponManager {
    pub weapons: Vec<Weapon>,
}

impl WeaponManager {
    pub fn new() -> Self {
        WeaponManager {
            weapons: vec![
                Weapon::new("Stick", 5, true, 0),             // Stick is unlocked by default
                Weapon::new("Slingshot", 7, true, 0),         // Slingshot is unlocked by default
                Weapon::new("Water Balloon", 0, true, 0),     // Water Balloon is unlocked by default
                Weapon::new("Pistol", 20, false, 10),         // Pistol unlocked with 10 points
                Weapon::new("Shotgun", 40, false, 20),        // Shotgun unlocked with 20 points
                Weapon::new("Grenade", 60, false, 30),         // Grenade unlocked with 30 points
                Weapon::new("C4", 100, false, 40),            // C4 unlocked with 40 points
                Weapon::new("Dubstep Gun", 30, false, 50),    // Dubstep Gun unlocked with 50 points
                Weapon::new("BFG", 200, false, 60),           // BFG unlocked with 60 points
                Weapon::new("Gravity Gun", 0, false, 100),    // Gravity Gun unlocked with 100 points
            ],
        }
    }

    pub fn unlock_weapons(&mut self, points: u32) {
        // Unlock weapons based on the points threshold
        for weapon in &mut self.weapons {
            if points >= weapon.unlock_threshold {
                weapon.unlocked = true;
            }
        }
    }

    pub fn display_available_weapons(&self) {
        println!("Choose your weapon:");
        for (index, weapon) in self.weapons.iter().enumerate() {
            if weapon.unlocked {
                println!("{}. {}", index + 1, weapon.name);
            }
        }
    }

    pub fn choose_weapon_by_index(&self, index: usize) -> Option<&Weapon> {
        self.weapons.get(index - 1) // Subtract 1 to account for 0-indexing
    }

    pub fn add_damage(&mut self, damage: u32) {
        // Additional logic to add damage
    }

    pub fn get_total_damage(&self) -> u32 {
        self.weapons.iter().filter(|w| w.unlocked).map(|w| w.damage).sum()
    }
}
