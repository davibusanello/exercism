pub fn raindrops(n: u32) -> String {
    get_sound(n)
}

fn get_sound(number: u32) -> String {
    let mut sound = String::new();
    if number % 3 == 0 {
        sound.push_str("Pling");
    }
    if number % 5 == 0 {
        sound.push_str("Plang");
    }
    if number % 7 == 0 {
        sound.push_str("Plong");
    }

    if sound.is_empty() {
        return format!("{}", number);
    }

    sound
}
