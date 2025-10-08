use crate::auth::authenticate_user;
use crate::cli::cli::cli_help_display;
use crate::ui::display::*;
use std::io::{self, Write};

pub fn main_menu() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        for arg in &args[1..] {
            match arg.as_str() {
                "--help" => {
                    cli_help_display();
                    std::process::exit(0);
                }
                "tasks" => {
                    let check_sub_command =
                        args.iter().find(|arg| *arg == "--view-completed").is_some();

                    if !check_sub_command {
                        eprintln!(
                            "\
                            [ERR] Invalid subcommand. \
                            please run help to pass in valid subcommand \
                        "
                        );
                        std::process::exit(1);
                    }

                    let get_username = args
                        .iter()
                        .position(|x| x == "--username")
                        .map(|x| &args[x + 1]);

                    let get_password = args
                        .iter()
                        .position(|x| x == "--password")
                        .map(|x| &args[x + 1]);

                    if get_username.is_none() || get_password.is_none() {
                        eprintln!(
                            "\
                            [ERR] Invalid username or password provided. \
                            please provide both for authentication \
                        "
                        );
                        std::process::exit(1);
                    }

                    let authenticated =
                        authenticate_user(get_username.unwrap(), get_password.unwrap());

                    if !authenticated {
                        eprintln!("User not found, please try again");
                        std::process::exit(1);
                    }

                    view_task(get_username.unwrap(), "completed");

                    std::process::exit(0);
                }
                _ => {
                    println!("command not found run --help for cli_help_display");
                    std::process::exit(1)
                }
            }
        }
    } else {
        loop {
            println!("\nMain Menu:");
            println!("[1] Create new user");
            println!("[2] Login");
            println!("[3] Exit");
            print!("-> Enter your choice [1-3]: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            match choice.trim() {
                "1" => handle_user_creation(),
                "2" => handle_login(),
                "3" => {
                    println!("Exiting... Goodbye!");
                    break;
                }
                _ => println!("Invalid choice. Please enter 1, 2, or 3."),
            }
        }
    }
}
