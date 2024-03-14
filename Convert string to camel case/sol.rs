fn to_camel_case(text: &str) -> String 
{
    let mut char_vec: Vec<_> = text.chars().collect();
    
    for i in 0..char_vec.len()
    {
        if char_vec[i] == '-' || char_vec[i] == '_'
        {
            char_vec[i + 1] = char_vec[i + 1].to_ascii_uppercase();
        }
    }
    
    let mut new_string: String = char_vec.into_iter().collect();
    new_string = new_string.replace("_", "");
    new_string = new_string.replace("-", "");

    new_string
}