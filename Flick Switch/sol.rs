fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut bool_vec: Vec<bool> = Vec::new();
    let mut switch = false;
    
    for i in list 
    {
        if i.to_string() == "flick"
        {
            switch = !switch;
        }

        if !switch
        {
            bool_vec.push(true);
            
            continue;
        }

        bool_vec.push(false);
    }

    bool_vec
}