use clap::{Arg, Command};
use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Game {
    kingdoms: HashMap<String, String>,
}

impl Game {
    fn new() -> Self {
        Self {
            kingdoms: HashMap::new(),
        }
    }

    fn add_kingdom(&mut self, name: &str, ruler: &str) {
        self.kingdoms.insert(name.to_string(), ruler.to_string());
    }

    fn list_kingdoms(&self) {
        for (kingdom, ruler) in &self.kingdoms {
            println!("{}: Ruled by {}", kingdom, ruler);
        }
    }

    fn battle(&mut self, kingdom: &str, challenger: &str) {
        let rng = rand::thread_rng().gen_range(0..=1);
        let current_ruler = self.kingdoms.get(kingdom);
        match current_ruler {
            Some(ruler) => {
                if rng == 1 {
                    println!("{} has overthrown {} in {}!", challenger, ruler, kingdom);
                    self.kingdoms.insert(kingdom.to_string(), challenger.to_string());
                } else {
                    println!("{} failed to conquer {} and {} retains control!", challenger, kingdom, ruler);
                }
            },
            None => println!("The kingdom of {} does not exist.", kingdom),
        }
    }

    fn save(&self) {
        let json = serde_json::to_string(&self).expect("Error serializing game state.");
        fs::write("game_state.json", json).expect("Error saving game state.");
    }

    fn load() -> Self {
        if Path::new("game_state.json").exists() {
            let json = fs::read_to_string("game_state.json").expect("Error loading game state.");
            serde_json::from_str(&json).expect("Error deserializing game state.")
        } else {
            Self::new()
        }
    }
}

fn main() {
    let matches = Command::new("Westeros Conquest")
        .version("1.0")
        .author("Nyakio")
        .about("Simulates conquests in the Game of Thrones universe")
        .subcommand(
            Command::new("list")
                .about("Lists all kingdoms and their rulers"),
        )
        .subcommand(
            Command::new("add")
                .about("Adds a new kingdom and its ruler")
                .arg(Arg::new("kingdom").help("The name of the kingdom").required(true))
                .arg(Arg::new("ruler").help("The ruler of the kingdom").required(true)),
        )
        .subcommand(
            Command::new("battle")
                .about("Challenge the ruler of a kingdom")
                .arg(Arg::new("kingdom").help("The name of the kingdom to challenge").required(true))
                .arg(Arg::new("challenger").help("The name of the challenger").required(true)),
        )
        .get_matches();

    let mut game = Game::load();

    match matches.subcommand() {
        Some(("list", _)) => {
            game.list_kingdoms();
        },
        Some(("add", add_matches)) => {
            let kingdom_name = add_matches.get_one::<String>("kingdom").expect("required");
            let ruler_name = add_matches.get_one::<String>("ruler").expect("required");
            game.add_kingdom(kingdom_name, ruler_name);
            println!("Added kingdom: {} ruled by {}", kingdom_name, ruler_name);
        },
        Some(("battle", battle_matches)) => {
            let kingdom_name = battle_matches.get_one::<String>("kingdom").expect("required");
            let challenger_name = battle_matches.get_one::<String>("challenger").expect("required");
            game.battle(kingdom_name, challenger_name);
        },
        _ => {
            println!("Westeros Conquest");
            println!("Use the following commands to play:");
            println!("list - Lists all kingdoms and their rulers-- make sure you have added the kingdoms and rulers first");
            println!("add <kingdom> <ruler> - Adds a new kingdom with its ruler");
            println!("battle <kingdom> <challenger> - Challenge the current ruler of a kingdom");
        },
    }

    game.save();
}
