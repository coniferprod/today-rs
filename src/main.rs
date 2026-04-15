use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::{NaiveDate, Local, Datelike};
use clap::{Parser, Subcommand};
use log;

use today::{run, add_event, Config, create_providers};
use today::events::{Event, Category, MonthDay};
use today::filters::FilterBuilder;

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// List all event providers
    Providers,

    /// Adds an event to an event provider
    Add {
        #[arg(short, long, help = "Name of event provider")]
        provider: String,

        #[arg(short, long, help = "Date of event. Format: YYYY-MM-DD")]
        date: String,

        #[arg(short = 'e', long, help = "Description of event")]
        description: String,

        #[arg(short, long, help = "Category of event. Format: primary[/secondary]")]
        category: String,
    }
}

#[derive(Parser)]
#[command(name = "today")]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,

    #[arg(short, long, help = "Event date in MMDD format")]
    date: Option<String>,

    #[arg(short, long, help = "Categories to exclude, comma-separated (a/b,c/d)")]
    exclude: Option<String>,

    #[arg(short, long, help = "No age calculation or birthday message")]
    no_birthday: bool,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    if !args.no_birthday {
        today::birthday::handle_birthday();
    }

    let month_day = if let Some(md) = args.date {
        MonthDay::from_str(&md)
    } else { 
        let today: NaiveDate = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };
    log::debug!("month_day = {}", month_day);

    // Handle the exclude categories option
    let mut categories: Vec<Category> = Vec::new();
    if let Some(exclude) = args.exclude {
        let parts: Vec<&str> = exclude.split(',').collect();
        for part in parts.iter() {
            let category = Category::from_str(part).unwrap();
            categories.push(category);
        }
        
        log::info!("Excluded categories:");
        for category in &categories {
            log::info!("- {}", category);
        }
        log::info!("These exclusions currently have no effect.");
    }

    let filter = FilterBuilder::new()
        .month_day(month_day)
        .build();
    //let filter = FilterBuilder::new().build();


    /*
    let filter = FilterBuilder::new()
        .category(Category::new("programming", "rust"))
        .text("released".to_string())
        .build();
 */

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            log::debug!("Looking for configuration file '{}'", &toml_path.display());
            let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
            let config: Config = toml::from_str(&config_str).expect("valid configuration file");
            log::debug!("config: {:#?}", config);

            match args.cmd {
                Some(Command::Providers) => {
                    let providers = create_providers(&config, &path);
                    for provider in providers {
                        println!(
                            "{}{}", 
                            provider.name(),
                            if provider.is_add_supported() { "*" } else { "" });                        
                    }
                },

                Some(Command::Add { provider, date, description, category }) => {
                    log::debug!("provider_name = '{}'  date = '{}'  description = '{}'  category = '{}'",
                        provider, date, description, category);
                    let category = Category::from_str(&category).unwrap();
                    let date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
                    let event = Event::new_singular(date, description, category);
                    add_event(&config, &path, &provider, &event);
                },

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
