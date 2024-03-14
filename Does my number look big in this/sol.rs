fn narcissistic(num: u64) -> bool 
{
    let num_string = num.to_string();
    let num_len = num_string.len();
    let num_chars: Vec<_> = num_string.chars().collect();
    let mut sum: u64 = 0;

    for i in 0..num_len
    {
        let new_num: u64;

        new_num = num_chars[i] as u64 - 0x30;

        sum += new_num.pow(num_len as u32);
    }

    sum == num
}