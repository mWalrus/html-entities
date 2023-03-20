use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use serde::Deserialize;
use serde_json;
use std::{fmt, fs, process::exit};

#[derive(Deserialize, Debug)]
struct Entity {
    html: String,
    symbol: String,
    numeric: String,
    title: String,
    hex: String,
    iso: String,
    octal: String,
}

impl Entity {
    pub fn searchable_string(&self) -> String {
        format!(
            "{title} - {html} (symbol: \"{symbol}\")",
            title = self.title,
            html = self.html,
            symbol = self.symbol
        )
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.title.bold().blue())?;
        write!(f, "{} {}\n", "html:".bold(), self.html)?;
        write!(f, "{} {}\n", "symbol:".bold(), self.symbol)?;
        write!(f, "{} {}\n", "numeric:".bold(), self.numeric)?;
        write!(f, "{} {}\n", "hex:".bold(), self.hex)?;
        write!(f, "{} {}\n", "iso:".bold(), self.iso)?;
        write!(f, "{} {}", "octal:".bold(), self.octal)?;
        Ok(())
    }
}

fn main() {
    let file_path = "/usr/share/html-entities/html-entities.json";
    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to read entity file at {file_path}: {err}");
        exit(1);
    });
    let entities = serde_json::from_str::<Vec<Entity>>(&contents).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to parse entity file: {err}");
        exit(1);
    });
    let searchable_entities: Vec<String> = entities.iter().map(Entity::searchable_string).collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Search for an entity")
        .items(&searchable_entities)
        .default(0)
        .interact()
        .unwrap();

    println!("{sel}", sel = entities.get(selection).unwrap());
}
