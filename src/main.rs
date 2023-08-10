use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    const HELP: &str = "\n\
        Alternative for Cargo\n\n\
        Usage: freight [COMMAND] [OPTIONS]\n\n\
        Commands:\n    \
            build    Build a Freight or Cargo project\n    \
            help     Print out this message
        ";

    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("build") => freight::build()?, // This is new
        Some("help") => println!("{HELP}"),
        _ => {
            println!("Unsupported command");
            println!("{HELP}");

            std::process::exit(1);
        }
    }

    Ok(())
}
