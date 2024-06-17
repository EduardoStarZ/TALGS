pub fn hex_str_spaced_to_u8_vec(str : String) -> Vec<u8> {
    str.trim_start_matches(" ").trim_end_matches(" ").split(" ").map(|x| match u8::from_str_radix(x, 16) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error while parsing a hexadecimal value to a unsigned integer of 8 bits! Returning a u8::MIN value and displaying backtrace: {err}"); 
            u8::MIN                                        
            }
    }
        ).collect::<Vec<u8>>()
}

pub fn unspaced_hex_str_to_u8_vec(str : String) -> Vec<u8> {
    let mut hexes : Vec<u8> = Vec::new();
    let mut chars : String = String::new();

    for (i, x) in str.chars().enumerate() {
        chars.push(x);

        if i+1 % 2 == 0 {
            chars = String::new();

            hexes.push(
                match u8::from_str_radix(&(*chars), 16) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("Error while parsing a hexadecimal value to a unsigned integer of 8 bits! Returning a u8::MIN value and displaying backtrace: {err}"); 
                        u8::MIN 
                    }
                }
            )
        }
    }
    return hexes;
}

pub fn u8_vec_to_hex_str(vector : Vec<u8>) -> String {
    vector.iter().map(|x| format!("{x:0>2b }")).collect::<String>()
}

pub fn unspaced_u8_vec_to_hex_str(vector : Vec<u8>) -> String {
    vector.iter().map(|x| format!("{x:0>2b}")).collect::<String>()
}
