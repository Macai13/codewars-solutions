fn spin_words(words: &str) -> String {
    let mut retstr: String = String::new();

    let words_vec = words.split(' ').collect::<Vec<&str>>();

    for word in words_vec
    {
        if word.len() >= 5
        {
            println!("{}", word);
            let temp_str = word.chars().rev().collect::<String>();
            retstr.push_str(&temp_str);
            retstr.push_str(" ");

            continue;
        }

        retstr.push_str(word);
        retstr.push_str(" ");
    }

    retstr.trim().to_string()
}