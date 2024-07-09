use core::panic;
use std::{
    env,
    error::Error,
    fs,
    process::{self, Command},
};

use image::imageops::FilterType;

const HELP_MESSAGE: &str = "
 Run this program in a directory with a `server.jar` and optionally an
 icon.png to set up a Minecraft Server.

 You can find the .jar file here:
 -> https://www.minecraft.net/en-us/download/server

 It is reccomended that the icon.png file is square as to not distort
 when making the server icon.

 Run the program with --icon if you already have a server setup to
 make an icon, provided the right icon.png file.

 For more info, visit this project's github page:
 -> https://github.com/outphase/mc-server-utility
";

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n Welcome to MC Server Setup!");

    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.get(1) {
        match arg.to_lowercase().trim() {
            "--help" => {
                println!("{HELP_MESSAGE}");
            }
            "--icon" => {
                if let Err(_) = make_icon() {
                    println!("\n Could not create icon\n Please provide an 'icon.png' file to set an icon");
                    // eprintln!(" Error creating icon: {e}");
                }
            }
            _ => println!(
                " Unknown argument '{arg}', run 'mc-server-setup --help' for usage information"
            ),
        }
        return Ok(());
    }

    if let Err(_e) = fs::read("./server.jar") {
        println!("{HELP_MESSAGE}");
        return Ok(());
    }
    if let Ok(_e) = fs::read("./start-server.bat") {
        println!(
            "
 This program can only be run once.
 If you wish to run this again, please remove everything 
 that is not `server.jar` and an eventual icon.png and try again.
"
        );
        return Ok(());
    }

    let mut min_ram_mb = Ok(4096);
    let mut max_ram_mb = Ok(4096);

    if read_y_n("\n Would you like to specify an amount of dedicated RAM? (y/n)\n Default 4096MB") {
        min_ram_mb =
            read_input("\n Please enter the amount of MINIMUM dedicated RAM in MB:").parse();
        while let Err(_) = min_ram_mb {
            min_ram_mb = read_input(" Please enter a valid number").parse();
        }

        max_ram_mb = if read_y_n("\n Keep maximum RAM the same as min RAM? (y/n)") {
            min_ram_mb.clone()
        } else {
            let mut result;
            result =
                read_input("\n Please enter the amount of MAXIMUM dedicated RAM in MB:").parse();
            // TODO: Should warn when min > max
            while let Err(_) = result {
                result = read_input(" Please enter a valid number:").parse();
            }
            result
        };
    }

    let bat_file_content = format!(
        "java -Xmx{}M -Xms{}M -jar server.jar nogui",
        min_ram_mb.expect("Should be a number by now"),
        max_ram_mb.expect("Should be a number by now")
    );

    let _ = fs::write("./start-server.bat", bat_file_content).unwrap_or_else(|e| {
        eprintln!(" Could not create .bat file: {e}");
        process::exit(1);
    });

    run_server_bat().unwrap_or_else(|e| {
        eprintln!(" Could not start server: {e}");
        process::exit(1);
    });

    println!("\n Accepting EULA...");
    fs::write("./eula.txt", "eula=true").unwrap_or_else(|e| {
        eprintln!(" Could not confirm EULA: {e}");
        process::exit(1);
    });

    if let Err(_) = make_icon() {
        println!(
            "
 Could not add an icon to your server.
 if you wish to add an icon, please provide an 'icon.png' file!
"
        );
    }

    if read_y_n("\n Would you like to run the server? (y/n)") {
        run_server_bat().expect("Could not run server");
    }

    Ok(())
}

fn make_icon() -> Result<(), Box<dyn Error>> {
    let image_name = "icon.png";
    img2ico::convert_image(image_name)?;
    let image = image::io::Reader::open(image_name)?.decode()?;
    let server_icon = image::DynamicImage::resize_exact(&image, 64, 64, FilterType::Nearest);
    let image_name = "server-icon.png";

    server_icon.clone().save(image_name)?;

    println!(" Created server-icon.png and icon.ico in current directory");
    Ok(())
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
    };

    Ok(())
}

fn read_y_n(say: &str) -> bool {
    match read_input(say).to_lowercase().trim() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            // HACK: Implement failstate
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
