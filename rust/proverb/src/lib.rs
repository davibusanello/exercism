pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    let mut index = 0;
    if list.len() == 0 { return proverb; }
    while index+1 < list.len() {
        let current_phrase = build_phrase(list[index], list[index + 1]);
        proverb.push_str(&current_phrase);
        index += 1;
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    return proverb;
}

fn build_phrase(first_word: &str, second_word: &str) -> String {
    format!(
        "For want of a {} the {} was lost.\n",
        first_word, second_word
    )
}
