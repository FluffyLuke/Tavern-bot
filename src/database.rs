use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use sqlx::SqlitePool;
use std::fs;
pub struct Database;

impl TypeMapKey for Database {
    type Value = Arc<RwLock<SqlitePool>>;
}

pub struct CommandDescriptions {
    pub descriptions: HashMap<String, String>,   
}

impl TypeMapKey for CommandDescriptions {
    type Value = Arc<RwLock<CommandDescriptions>>;
}

impl CommandDescriptions {
    pub fn new(command_descriptions: HashMap<String, String>) -> CommandDescriptions {
        CommandDescriptions { descriptions: command_descriptions }
    }
}

pub fn split_at(character: char, file_path: &str) -> Result<HashMap<String, String>, String> {
    let file = fs::read_to_string(file_path);
    let content = match file {
        Err(err) => return Err(err.to_string()),
        Ok(content) => content,
    };
    let mut hashmap = HashMap::new();
    for line in content.lines() {
        let semicolon_position = line.find(character);
        if let None = semicolon_position {
            return Err(format!("Syntax error while retrieving a hashmap from a file, {}", file_path));
        }
        let semicolon_positon = semicolon_position.unwrap();
        let (command_name, command_description) = line.split_at(semicolon_positon);
        hashmap.insert(command_name.to_string(), command_description.strip_prefix(';').unwrap().to_string());
    }
    Ok(hashmap)
}
