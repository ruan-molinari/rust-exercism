pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = String::new();
    for idx in ((start_bottles - take_down + 1)..(start_bottles + 1)).rev() {
        let number = get_number_text(idx);
        verses.push_str(
            format!(
                "{} green {} hanging on the wall,\n",
                number,
                bottle_or_bottles(idx)
            )
            .as_str(),
        );
        verses.push_str(
            format!(
                "{} green {} hanging on the wall,\n",
                number,
                bottle_or_bottles(idx)
            )
            .as_str(),
        );
        verses.push_str("And if one green bottle should accidentally fall,\n");
        verses.push_str(
            format!(
                "There'll be {} green {} hanging on the wall.\n",
                get_number_text(idx - 1).to_lowercase(),
                bottle_or_bottles(idx - 1)
            )
            .as_str(),
        );
        verses.push('\n');
    }

    verses.trim().to_string()
}

fn bottle_or_bottles(number: u32) -> String {
    match number {
        1 => "bottle".to_string(),
        _ => "bottles".to_string(),
    }
}

fn get_number_text(number: u32) -> String {
    match number {
        0 => "No".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        _ => panic!("Can't be a number higher than 10"),
    }
}
