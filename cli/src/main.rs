mod lib;

use std::env; // used to extract the arguments from cli

fn main() {
    // collects the arguments as a vector of strings.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    match query.as_str(){
        "R" => lib::read_file(&args),
        "W" => lib::write_to_file(&args),
        "C" => lib::create_file(&args),
        "find" => lib::find_all_instances(&args),
        "size" => lib::size(&args),
        "rm" => lib::remove_file(&args),
        _ => println!("something went wrong")
    }
}
