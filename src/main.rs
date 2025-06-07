mod commands;
mod files;

use commands::{append_film_viewing, list_film_viewings, print_commands};
use dotenv::dotenv;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    dotenv().ok();

    let not_path = env::var("NOT_PATH").expect("NOT_PATH must be set in the .env file");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_commands();
        return Ok(());
    }

    match args[1].as_str() {
        "view" | "v" => {
            if args.len() < 3 {
                list_film_viewings(not_path.clone().into())?;
                return Ok(());
            }
            let viewing_time = if args.len() > 3 { Some(&args[3]) } else { None };
            append_film_viewing(not_path.into(), &args[2], viewing_time.map(|x| x.as_str()))
        }
        &_ => {
            print_commands();
            Ok(())
        }
    }
}
