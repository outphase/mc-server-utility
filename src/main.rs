use core::panic;
use image::imageops::FilterType;
use std::{error::Error, fs, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to MineServe!\n");

    if let Err(_e) = fs::read("./server.jar") {
        println!(
            " Please run this in a directory with a server.jar\n\
             - https://www.minecraft.net/en-us/download/server - \n"
        );
        return Ok(());
    }
    if let Ok(_e) = fs::read("./start-server.bat") {
        println!(
            " This program can only be run once.\
             Please remove all that is not `server.jar` and try again."
        );
        return Ok(());
    }

    let mut min_ram_mb = 4096;
    let mut max_ram_mb = 4096;

    if read_y_n("\n Would you like to specify an amount of dedicated RAM? (y/n)\n Default 4096MB") {
        // TODO: Implement failstate
        min_ram_mb = read_input("\n Please enter the amount of MINIMUM dedicated RAM in MB:")
            .parse()
            .expect("This is not a number");
        max_ram_mb = if read_y_n("\n Keep max RAM the same as min RAM? (y/n)") {
            min_ram_mb
        } else {
            read_input("\n Please enter the amount of MAXIMUM dedicated RAM in MB:")
                .parse()
                .expect("This is not a number")
        };
    }

    let bat_file_content = format!(
        "java -Xmx{}M -Xms{}M -jar server.jar nogui",
        min_ram_mb, max_ram_mb
    );

    let _ = fs::write("./start-server.bat", bat_file_content).expect("Could not create bat file");

    run_server_bat().expect("Could not run server");

    // TODO: Implement Failstate
    println!("\n Accepting EULA...");
    fs::write("./eula.txt", "eula=true").expect("Could not find eula file");

    // TODO: Implement autodetection
    // TODO: Modularize
    if let Ok(_) = fs::read("./icon.png") {
        println!("\n Creating icons..,");
        make_icons();
    }

    if read_y_n("\n Would you like to run the server?") {
        run_server_bat().expect("Could not run server");
    }

    Ok(())
}

fn make_icons() {
    // TODO: Implement Failstate
    let image = image::io::Reader::open("./icon.png")
        .expect("Couldnt load image")
        .decode()
        .expect("could not decode image");
    let server_icon = image::DynamicImage::resize_exact(&image, 64, 64, FilterType::Nearest);
    server_icon
        .clone()
        .save("./server-icon.png")
        .expect("Could not save");
    server_icon
        .clone()
        .save("./icon.ico")
        .expect("could not save icon");
}

fn run_server_bat() -> Result<(), Box<dyn Error>> {
    let bat_command = Command::new("./start-server.bat")
        .spawn()
        .expect("Could not run start-server.bat file");
    let output = bat_command.wait_with_output();
    if let Ok(output) = output {
        let stderr = String::from_utf8(output.stderr).expect("Could not pars stderr");
        let stdout = String::from_utf8(output.stdout).expect("Could not parse stdout");
        println!("{stdout}{stderr}");
        // if output.status.to_string().trim() != "server.jar errored" || !stderr.is_empty() {
        //     return Err("Could not run .jar".into());
        // }
    };

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
