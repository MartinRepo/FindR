use anyhow::Result;
use clap::Parser;
use findme::{
    analyze_dev_pressure, display_dev_pressure, display_fortune, get_language_choice, i18n,
    Language,
};
use std::fs;

#[derive(Parser)]
#[command(name = "findme")]
#[command(
    about = "Developer's Daily Decompression Oracle - Tech dimension analysis with deterministic daily variations"
)]
#[command(version)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    language: Option<String>,

    #[arg(long)]
    set_language: bool,

    #[arg(short, long)]
    birthday: Option<String>,

    #[arg(long)]
    pressure: bool,
}

#[derive(Default, Debug)]
struct UserConfig {
    language: Option<Language>,
    birthday: Option<String>,
}

fn get_config_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("FINDME_CONFIG_DIR") {
        return std::path::PathBuf::from(dir);
    }

    let mut config_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    config_dir.push(".findme");
    config_dir
}

fn get_config_file() -> std::path::PathBuf {
    let mut config_file = get_config_dir();
    config_file.push("config.txt");
    config_file
}

fn load_config() -> UserConfig {
    let config_file = get_config_file();
    if let Ok(content) = fs::read_to_string(config_file) {
        parse_config(&content)
    } else {
        UserConfig::default()
    }
}

fn parse_config(content: &str) -> UserConfig {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return UserConfig::default();
    }

    match trimmed {
        "zh" | "chinese" => UserConfig {
            language: Some(Language::Chinese),
            birthday: None,
        },
        "en" | "english" => UserConfig {
            language: Some(Language::English),
            birthday: None,
        },
        _ => {
            let mut config = UserConfig::default();
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim();
                    match key {
                        "language" => match value.to_lowercase().as_str() {
                            "zh" | "chinese" => config.language = Some(Language::Chinese),
                            "en" | "english" => config.language = Some(Language::English),
                            _ => {}
                        },
                        "birthday" => {
                            if !value.is_empty()
                                && chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").is_ok()
                            {
                                config.birthday = Some(value.to_string());
                            }
                        }
                        _ => {}
                    }
                }
            }
            config
        }
    }
}

fn save_config(config: &UserConfig) -> Result<()> {
    let config_dir = get_config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_file = get_config_file();
    let mut lines = Vec::new();

    if let Some(lang) = config.language {
        let lang_value = match lang {
            Language::Chinese => "zh",
            Language::English => "en",
        };
        lines.push(format!("language={}", lang_value));
    }

    if let Some(birthday) = &config.birthday {
        lines.push(format!("birthday={}", birthday));
    }

    let content = lines.join("\n");
    fs::write(config_file, content)?;
    Ok(())
}

fn load_language() -> Option<Language> {
    load_config().language
}

fn save_language(lang: Language) -> Result<()> {
    let mut config = load_config();
    config.language = Some(lang);
    save_config(&config)
}

fn parse_language(lang_str: &str) -> Option<Language> {
    match lang_str.to_lowercase().as_str() {
        "zh" | "chinese" => Some(Language::Chinese),
        "en" | "english" => Some(Language::English),
        _ => {
            // Check against i18n translations
            if lang_str == i18n("lang.chinese", Language::Chinese) {
                Some(Language::Chinese)
            } else if lang_str == i18n("lang.english", Language::Chinese) {
                Some(Language::English)
            } else {
                None
            }
        }
    }
}

fn load_birthday() -> Option<String> {
    load_config().birthday
}

fn save_birthday(birthday: &str) -> Result<()> {
    let mut config = load_config();
    config.birthday = Some(birthday.trim().to_string());
    save_config(&config)
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.set_language {
        let lang = get_language_choice();
        save_language(lang)?;
        match lang {
            Language::Chinese => println!("{}", i18n("language.set_chinese", lang)),
            Language::English => println!("{}", i18n("language.set_english", lang)),
        }
        return Ok(());
    }

    let language = if let Some(lang_str) = args.language {
        parse_language(&lang_str).unwrap_or_else(|| {
            eprintln!(
                "{}",
                i18n("language.invalid_option", Language::Chinese).replace("{}", &lang_str)
            );
            eprintln!("{}", i18n("language.use_zh_en", Language::Chinese));
            std::process::exit(1);
        })
    } else {
        load_language().unwrap_or_else(|| {
            println!("{}", i18n("language.first_time", Language::Chinese));
            let lang = get_language_choice();
            if let Err(e) = save_language(lang) {
                eprintln!(
                    "{}",
                    i18n("language.cannot_save", Language::Chinese).replace("{}", &e.to_string())
                );
            }
            lang
        })
    };

    if args.verbose {
        println!("🎯 Findme - {}", i18n("app.title", language));
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
        println!();
    }

    let birthday = if let Some(birthday_str) = args.birthday.as_deref() {
        if chrono::NaiveDate::parse_from_str(birthday_str, "%Y-%m-%d").is_err() {
            eprintln!(
                "Invalid birthday format: {}. Please use YYYY-MM-DD format",
                birthday_str
            );
            std::process::exit(1);
        }
        if let Err(e) = save_birthday(birthday_str) {
            eprintln!("⚠️ Could not save birthday: {}", e);
        }
        Some(birthday_str.to_string())
    } else {
        load_birthday()
    };

    let fortune = if let Some(birthday_str) = birthday.as_deref() {
        findme::generate_daily_fortune_with_birthday(birthday_str, language)
    } else {
        findme::generate_daily_fortune(language)
    };

    display_fortune(&fortune, language);

    if args.pressure {
        match analyze_dev_pressure(language) {
            Ok(pressure) => display_dev_pressure(&pressure, language),
            Err(e) => {
                eprintln!("⚠️ Failed to analyze developer pressure: {}", e);
                eprintln!("💡 Make sure you're in a git repository and have cargo available");
            }
        }
    }

    Ok(())
}
