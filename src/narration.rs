use rand::Rng;
use std::{thread, time};
use std::io::Write;

/// Function to simulate gradual text generation
pub fn generate_typing_effect(text: &str) {
    let delay = time::Duration::from_millis(50); // Adjust speed of text
    for c in text.chars() {
        print!("{}", c);
        std::io::stdout().flush().unwrap();
        thread::sleep(delay);
    }
    println!(); // Move to the next line after printing the text
}

pub fn generate_damage_reaction(buddy_name: &str, weapon_name: &str) -> String {
    let mut rng = rand::thread_rng();
    match weapon_name {
        "Pistol" => {
            let reactions = vec![
                format!("{} winces as the bullet strikes, their body jerking back from the impact.", buddy_name),
                format!("{} staggers backward, clutching their shoulder, clearly hurt by the precision shot.", buddy_name),
                format!("{} gasps as the bullet hits them, the sting of pain making them wince.", buddy_name),
            ];
            reactions[rng.gen_range(0..reactions.len())].clone()
        }
        "Slingshot" => {
            let reactions = vec![
                format!("{} rubs their forehead where the stone hit, cursing under their breath.", buddy_name),
                format!("{} staggers back, clearly stunned from the hit, and glares at you.", buddy_name),
                format!("{} shakes their head, trying to regain balance after the stone to the face.", buddy_name),
            ];
            reactions[rng.gen_range(0..reactions.len())].clone()
        }
        "Water Balloon" => {
            let reactions = vec![
                format!("{} laughs and shakes the water off, clearly unfazed by the attack.", buddy_name),
                format!("{} looks up, dripping wet, and chuckles, \"That’s the best you’ve got?\"", buddy_name),
                format!("{} shakes their head, water dripping from their hair, clearly unimpressed.", buddy_name),
            ];
            reactions[rng.gen_range(0..reactions.len())].clone()
        }
        "Shotgun" => {
            let reactions = vec![
                format!("{} is knocked off their feet by the shotgun blast, groaning in pain as they struggle to stand.", buddy_name),
                format!("{} stumbles backward, their chest heaving from the impact of the shotgun's force.", buddy_name),
                format!("{} hits the ground with a loud thud, struggling to get back up after the blast.", buddy_name),
            ];
            reactions[rng.gen_range(0..reactions.len())].clone()
        }
        "Dubstep Gun" => {
            let reactions = vec![
                format!("{} stumbles, disoriented by the pounding bass. Their vision blurs as the soundwave hits.", buddy_name),
                format!("{} staggers back, clutching their ears, the intense bass making their knees weak.", buddy_name),
                format!("{} wobbles, their mind reeling from the bass-induced disorientation.", buddy_name),
            ];
            reactions[rng.gen_range(0..reactions.len())].clone()
        }
        "Gatling Gun" => {
            let reactions = vec![
                format!("{} is pummeled by the barrage of bullets, barely able to remain conscious.", buddy_name),
                format!("{} drops to their knees, blood pouring from multiple wounds as they try to shield themselves.", buddy_name),
                format!("{} falls to the ground, barely able to breathe as the gunfire tears through them.", buddy_name),
            ];
            reactions[rng.gen_range(0..reactions.len())].clone()
        }
        "Rocket Launcher" => {
            let reactions = vec![
                format!("{} is thrown back by the explosion, their body slamming into the wall with a sickening thud.", buddy_name),
                format!("{} groans as the rocket blast sends them sprawling across the room, covered in dust and debris.", buddy_name),
                format!("{} stumbles forward, their body scorched by the blast, but somehow still standing.", buddy_name),
            ];
            reactions[rng.gen_range(0..reactions.len())].clone()
        }
        _ => {
            let reactions = vec![
                format!("{} stumbles back, clearly hurt by the attack, but not as much as expected.", buddy_name),
                format!("{} looks confused, but tries to shake off the minor injury.", buddy_name),
                format!("{} winces, but then shrugs it off, not too bothered by the attack.", buddy_name),
            ];
            reactions[rng.gen_range(0..reactions.len())].clone()
        }
    }
}

pub fn generate_weapon_use_narration(weapon_name: &str, buddy_name: &str) -> String {
    let mut rng = rand::thread_rng();
    match weapon_name {
        "Pistol" => {
            let phrases = vec![
                format!("You draw the {} and aim carefully at {}. With a quick shot, the bullet grazes them, leaving a faint burn mark.", weapon_name, buddy_name),
                format!("You fire the {} at {}. The shot lands with precision, making them stagger back.", weapon_name, buddy_name),
                format!("A sharp crack echoes as you fire the {} at {}. The bullet strikes them square in the shoulder!", weapon_name, buddy_name),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
        "Slingshot" => {
            let phrases = vec![
                format!("You pull back the slingshot and release a stone that nails {} right in the forehead! They stumble back, dazed.", buddy_name),
                format!("With a quick flick, you launch a stone at {} with the slingshot! It hits them square in the chest, knocking the wind out.", buddy_name),
                format!("You aim and fire the slingshot. The stone strikes {} right in the nose! A loud crack echoes.", buddy_name),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
        "Water Balloon" => {
            let phrases = vec![
                format!("You lob the {} at {}. The balloon bursts on impact, drenching them in water. They laugh it off, clearly unfazed.", weapon_name, buddy_name),
                format!("You toss the {} and it explodes with a splash! {} wipes their face and grins, unimpressed.", weapon_name, buddy_name),
                format!("You throw the {} at {}. They catch it in mid-air and pop it, drenching you in return!", weapon_name, buddy_name),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
        "Shotgun" => {
            let phrases = vec![
                format!("You aim the {} and fire. The blast sends {} flying back, a loud *BOOM* ringing in your ears.", weapon_name, buddy_name),
                format!("With a deafening roar, you blast {} with the shotgun. They stumble back, barely keeping their footing. {:?}", weapon_name, buddy_name),
                format!("The {} erupts with force, sending a hail of pellets at {}. They crash to the ground, covered in dirt and smoke.", weapon_name, buddy_name),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
        "Dubstep Gun" => {
            let phrases = vec![
                format!("You fire the {}. The air is filled with thumping bass as the soundwave slams into {}! They collapse to their knees, disoriented.", weapon_name, buddy_name),
                format!("With a twist of the dial, the {} launches a barrage of booming bass at {}. They stagger back, unable to process the shock.", weapon_name, buddy_name),
                format!("You unleash the full power of the {} on {}. The soundwaves hit them like a freight train, sending them into a trance-like state.", weapon_name, buddy_name),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
        "Gatling Gun" => {
            let phrases = vec![
                format!("You unleash a hailstorm of bullets from the {}. {} is shredded by the rapid-fire, barely able to keep up with the onslaught.", weapon_name, buddy_name),
                format!("You fire the {}. The world seems to explode in a fury of gunfire, and {} is caught in the crossfire.", weapon_name, buddy_name),
                format!("The {} roars to life, sending a barrage of bullets at {}. Their body is torn apart by the relentless fire.", weapon_name, buddy_name),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
        "Rocket Launcher" => {
            let phrases = vec![
                format!("You fire the {} at {}. A huge explosion sends them flying across the room, leaving a smoking crater in their wake.", weapon_name, buddy_name),
                format!("The {} launches with a deafening boom, and the shockwave blasts {} off their feet, sending them skidding across the floor.", weapon_name, buddy_name),
                format!("With a massive explosion, you fire the {}. The blast knocks {} back into the walls, barely escaping the fiery aftermath.", weapon_name, buddy_name),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
        _ => {
            let phrases = vec![
                format!("You use the {} on {}. It’s not very effective, but at least it makes a big noise.", weapon_name, buddy_name),
                format!("You attempt to use the {}. {} flinches, unsure of what just happened.", weapon_name, buddy_name),
                format!("You unleash the {} at {}. It’s a bit anticlimactic, but it gets the job done.", weapon_name, buddy_name),
            ];
            phrases[rng.gen_range(0..phrases.len())].clone()
        }
    }
}
fn random_phrase(phrases: &[String]) -> Option<String> {
    if phrases.is_empty() {
        None // Handle empty array safely
    } else {
        let mut rng = rand::thread_rng();
        Some(phrases[rng.gen_range(0..phrases.len())].clone())
    }
}