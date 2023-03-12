use std::io;
//New way to make inputs for Rust(not really though)
pub fn return_int() -> i32{
    /*
    This function makes it easy for the user to
    enter a number and expect a integer
    */
    let mut newline = String::new();
    io::stdin()
    .read_line(&mut newline)
    .expect("Failed to return line");
    //Conversion from string to int(or float)
    let input: i32 = newline
    .trim()
    .parse()
    .expect("Failed to take in a number");
    return input;

}

pub fn return_float() -> f32{
    let mut newline = String::new();
    io::stdin()
    .read_line(&mut newline)
    .expect("Failed to return line");
    //Conversion from string to int(or float)
    let input: f32 = newline
    .trim()
    .parse()
    .expect("Failed to take in a number");
    return input;
}

// pub fn return_string() -> String{
//     /*
//     This function makes it easy for the user to
//     enter a number and expect a string
//     */
//     let mut newline = String::new();
//     io::stdin()
//     .read_line(&mut newline)
//     .expect("Failed to return line");
//     return newline;
// }
