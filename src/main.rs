use core::panic;
use std::{error::Error, fs, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to MineServe!\nPlease enter a command:\n");

    if let Err(e) = fs::read("./server.jar") {
        println!(
            " Please run this in a directory with a server.jar\n\
             - https://www.minecraft.net/en-us/download/server - \n\n"
        );
        return Ok(());
    }

    let mut min_ram_mb = 4096;
    let mut max_ram_mb = 4096;
    let mut bat_file_content = String::new();

    if read_y_n(" Would you like to specify an amount of dedicated RAM? (y/n)\n") {
        // TODO: Implement failstate
        min_ram_mb = read_input(" Please enter the amount of dedicated RAM in MB:\n")
            .parse()
            .expect("This is not a number");
        max_ram_mb = if read_y_n(" Keep max RAM the same as min RAM? (y/n)\n") {
            min_ram_mb
        } else {
            read_input("").parse().expect("This is not a number")
        };
    }

    bat_file_content = format!(
        "java -Xmx{}M -Xms{}M -jar server.jar nogui",
        min_ram_mb, max_ram_mb
    );

    let _ = fs::write("./start-server.bat", bat_file_content).expect("Could not create bat file");
    // let bat_command = Command::new("./start-server.bat").spawn();

    Ok(())
}

fn read_y_n(say: &str) -> bool {
    match read_input(say).to_lowercase().trim() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            // TODO: Implement failstate
            panic!("Enter y or n");
        }
    }
}

fn read_input(say: &str) -> String {
    println!("{}", say);
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap_or_else(|_e| {
        println!("  Could not read command, try again.");
        0
    });
    input.trim().to_string()
}
