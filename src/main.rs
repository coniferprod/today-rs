//mod events;
//mod providers;
//pub mod filters;
// See https://users.rust-lang.org/t/crate-compiled-multiple-times-and-type-with-similar-name/110928

use std::fs;
use std::path::PathBuf;
use today::{run, Config};
use today::filters::FilterBuilder;
use today::events::MonthDay;
use chrono::{NaiveDate, Local, Datelike};
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// List all event providers
    Providers,

    /// Adds an event to an event provider
    Add,
}

#[derive(Parser)]
#[command(name = "today")]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,

    #[arg(short, long, help = "Event date in MMDD format")]
    date: Option<String>,
}

fn main() {
    let args = Args::parse();

    let month_day = if let Some(md) = args.date {
        MonthDay::from_str(&md)
    } else { 
        let today: NaiveDate = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };
    let filter = FilterBuilder::new()
        .month_day(month_day)
        .build();

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            println!("Looking for config file'{}'", &toml_path.display());
            let config_str = fs::read_to_string(toml_path).expect("config file");
            let config: Config = toml::from_str(&config_str).expect("valid config file");
            //println!("config: {:?}", config);           

            match args.cmd {
                Some(Command::Providers) => {
                    for provider in config.providers {
                        println!("{}", provider.name);
                    }
                },

                Some(Command::Add) => todo!("add not implemented yet"),

                _ => {
                    if let Err(e) = run(&config, &path, &filter) {
                        eprintln!("Error running program: {}", e);
                        return;
                    }
                }
            }
        }
        None => {
            eprintln!("Unable to configure the application");
            return;
        }
    }
}

fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join(app_name);
        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            }
        }
        return Some(config_path);
    }
    None
}
