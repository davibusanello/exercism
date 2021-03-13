pub fn verse(n: u32) -> String {
    if n == 0 {
        let str = "No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n";
        return String::from(str);
    }

    let fist_line_unit = if n > 1 { "bottles" } else { "bottle" };
    let first_line = format!(
        "{} {} of beer on the wall, {} {} of beer.",
        n, fist_line_unit, n, fist_line_unit
    );

    let remaining = n - 1;
    let take_string = if n > 1 { "one" } else { "it" };
    let remaining_string = if remaining == 0 {
        String::from("no more bottles")
    } else if remaining == 1 {
        String::from("1 bottle")
    } else {
        format!("{} bottles", remaining.clone())
    };
    let second_line = format!(
        "Take {} down and pass it around, {} of beer on the wall.",
        take_string, remaining_string
    );

    format!("{}\n{}\n", first_line, second_line)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut sing = "".to_string();
    for number in end..=start {
        let verses = verse(number);
        if number != end {
            sing.insert(0, '\n')
        }
        sing.insert_str(0, verses.as_str());
    }

    sing
}
