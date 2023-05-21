use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("You must provide one argument: 'start' or 'end'!");
        std::process::exit(1);
    }

    let argument = &args[1];
    
    if argument != "start" && argument != "end" {
        println!("The argument must be either 'start' or 'end'!");
        std::process::exit(1);
    }

    let current_time = Utc::now();

    let file_content = if argument == "end" {
        format!("  {}: {}\n\n", argument, current_time)
    } else {
        format!("{}: {}\n", argument, current_time)
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("time.txt")?;
    file.write_all(file_content.as_bytes())?;

    Ok(())
}
