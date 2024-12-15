// Define a struct to represent the Buddy character
pub struct Buddy {
    pub name: String,  // Name of the Buddy
    pub health: u32,   // Health points of the Buddy
    pub points: u32,   // Points to track the Buddy's progress
}

impl Buddy {
    // Constructor to create a new Buddy instance
    pub fn new(name: &str, health: u32) -> Self {
        Buddy {
            name: name.to_string(), // Convert the name string to a String
            health,                // Set the initial health
            points: 0,             // Start with 0 points
        }
    }

    // Function to increase the Buddy's points when they gain points (e.g., from using a weapon)
    pub fn gain_points(&mut self, points: u32) {
        self.points += points;  // Add the points to the Buddy's total points
        println!("{} gains {} points!", self.name, points); // Print the points gained message
    }

    // Function to reduce the Buddy's health when they take damage
    pub fn take_damage(&mut self, damage: u32) {
        if self.health > damage {  // If health is greater than damage
            self.health -= damage;  // Subtract the damage from health
        } else {
            self.health = 0; // Buddy is defeated if health reaches 0 or lower
        }
    }

    // Function to revive the Buddy, restoring their health to a maximum value
    pub fn revive(&mut self) {
        self.health = 100;  // Set health to 100 when the Buddy is revived
    }

    // Function to simulate the Buddy resting or idling
    pub fn random_idle(&self) {
        println!("{} is resting...", self.name);  // Print a message showing the Buddy is resting
    }
}
