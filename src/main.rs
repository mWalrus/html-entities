use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use serde::Deserialize;
use serde_json;
use std::{fmt, fs};

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
        format!("{} - {}", self.title, self.html)
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

fn main() -> Result<()> {
    let contents = fs::read_to_string("/usr/share/html-entities/html-entities.json")?;
    let entities = serde_json::from_str::<Vec<Entity>>(&contents)?;

    let searchable_entities: Vec<String> = entities.iter().map(Entity::searchable_string).collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Search for an entity")
        .items(&searchable_entities)
        .default(0)
        .interact()
        .unwrap();

    println!("{}", entities.get(selection).unwrap());

    Ok(())
}
