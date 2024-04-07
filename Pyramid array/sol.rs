fn pyramid(n: usize) -> Vec<Vec<u32>> {
    let mut retvec: Vec<Vec<u32>> = Vec::new();
    
    for i in 1..=n
    {
        let new_vec: Vec<u32> = vec![1; i];

        retvec.push(new_vec);
    }

    retvec
}