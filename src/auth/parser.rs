

///This function takes up a reference to a String with spaces between the hex values and returns a
///bit vector
pub fn hex_str_spaced_to_u8_vec(str : &String) -> Vec<u8> {
    str.trim_start_matches(" ").trim_end_matches(" ").split(" ").map(|x| match u8::from_str_radix(x, 16) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error while parsing a hexadecimal value to a unsigned integer of 8 bits! Returning a u8::MIN value and displaying backtrace: {err}"); 
            u8::MIN                                        
            }
    }
        ).collect::<Vec<u8>>()
}


///This function takes up a reference to a String without spaces between the hex values and returns
///a bit vector
pub fn unspaced_hex_str_to_u8_vec(str : &String) -> Vec<u8> {
    let mut hexes : Vec<u8> = Vec::new();
    let mut copied_str = str.clone().chars().rev().collect::<String>();

    for _ in 0..(str.len() / 2) {
        let sub_str : String = format!("{}{}", copied_str.pop().unwrap(), copied_str.pop().unwrap());

        let value : u8 = match u8::from_str_radix(sub_str.as_str(), 16) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Error while parsing a hexadecimal value to a unsigned integer of 8 bits! Returning a u8::MIN value and displaying backtrace: {err}"); 
                u8::MIN 
            }
        };
        hexes.push(value);
    }
    return hexes;
}


///This function takes up a reference to a bit vector and returns a String with spaced hex values
pub fn u8_vec_to_hex_str(vector : &Vec<u8>) -> String {
    vector.iter().map(|x| format!("{x:0>2x} ")).collect::<String>()
}


//This function takes up a reference to a bit vector and returns a String with glued up hex values
pub fn unspaced_u8_vec_to_hex_str(vector : &Vec<u8>) -> String {
    vector.iter().map(|x| format!("{x:0>2x}")).collect::<String>()
}
