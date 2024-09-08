
use clap::{*};
use clap::builder::{*};
use std::io::{self, *};
use serde::{*};
use serde_json;



#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    is_employed: bool,
}

#[derive(Subcommand)]
enum CommandCLI {
    Serial {
        #[arg(long)]
        name: String,
        #[arg(long)]
        age: u8,
        #[arg(long)]
        is_employed: bool,
    },
    Deserial,
}

#[derive(clap::Parser)]
#[command(name = "cluster", version = "1.0", author = "Terry Fisher <terry.fisher.dev@protonmail.com>", about = "Reusable stdin/stdout")]
struct Cli {
    #[command(subcommand)]
    command: CommandCLI,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        CommandCLI::Serial { name, age, is_employed } => {
            let person = Person {
                name: name.to_string(),
                age: *age,
                is_employed: *is_employed,
            };

            match serde_json::to_string(&person) {
                Ok(serialized) => println!("{}", serialized),
                Err(e) => eprintln!("Error serializing data: {}", e),
                // Ok(serialized) => io::stdout().write_all(serialized.as_bytes()),
                // Err(e) => io::stderr().write_all(format!("Error serializing data: {}", e).as_bytes()),
            };
        }
        CommandCLI::Deserial => {
            let mut input = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut input) {
                eprintln!("Failed to read from stdin: {}", e);
                return;
            }

            match serde_json::from_str::<Person>(&input) {
                Ok(mut person) => {
                    person.age += 10;
                    person.is_employed = !person.is_employed;

                    println!("{}", serde_json::to_string(&person).unwrap());
                },
                Err(e) => eprintln!("Error serializing data: {}", e),
                // Ok(person) => io::stdout().write_all(serde_json::to_string(&person).unwrap().as_bytes()),
                // Err(e) => io::stderr().write_all(format!("Error serializing data: {}", e).as_bytes()),
            };
        }
    }
}





// fn main() {
//     println!("Hello, world!");
// }
