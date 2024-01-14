use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);  this command can be used to print the value of args(vector)
    let query = &args[1]; // the program's name takes up the first value in args
    let file_path = &args[2];  // that's why [1] and [2] are used
}
