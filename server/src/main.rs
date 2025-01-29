mod deck;
mod database;

use crate::database::Database;
use crate::deck::Deck;
use std::io;

fn main() -> rusqlite::Result<()> {
    let db = Database::new("players.db")?; // Initialize SQLite database
    let mut logged_in_player: Option<(String, String)> = None; // Track logged in player (ID, Name)

    // Handle player registration or login
    println!("=== Welcome to Poker Game ===");
    loop {
        if logged_in_player.is_none() {
            println!("\nChoose an option: \n1. Sign Up \n2. Sign In \n3. Quit");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");
            let choice = choice.trim();

            match choice {
                "1" => {
                    // Player Sign Up
                    println!("=== Player Sign Up ===");
                    println!("Enter a username to sign up:");
                    let mut name = String::new();
                    io::stdin().read_line(&mut name).expect("Failed to read input");
                    let name = name.trim();

                    let id = db.register_player(name)?;
                    println!("Player '{}' registered with ID: {}", name, id);
                    logged_in_player = Some((id, name.to_string())); // Set the logged-in player with String id
                    break; // Proceed to the game after signing up
                }
                "2" => {
                    // Player Sign In
                    println!("=== Player Sign In ===");
                    println!("Enter your username to sign in:");
                    let mut name = String::new();
                    io::stdin().read_line(&mut name).expect("Failed to read input");
                    let name = name.trim();

                    // Attempt to log in by checking if the player exists in the database
                    if let Some(id) = db.login_player(name)? {
                        println!("Welcome back, {}! You are logged in with ID: {}", name, id);
                        logged_in_player = Some((id, name.to_string())); // Set the logged-in player with String id
                        break; // Proceed to the game after signing in
                    } else {
                        println!("Player '{}' not found. Please sign up first.", name);
                    }
                }
                "3" => {
                    println!("Goodbye!");
                    return Ok(()); // Exit the program
                }
                _ => println!("Invalid option. Please try again."),
            }
        } else {
            // A player is logged in, proceed with the game setup
            println!("\nLogged in as: {}", logged_in_player.as_ref().unwrap().1);
            break;
        }
    }

    // List registered players
    println!("\n=== Registered Players ===");
    for player in db.list_players()? {
        println!("ID: {}, Name: {}", player.id, player.name);
    }


    // Create and shuffle the deck
    let mut deck = deck::Deck::new();
    deck.shuffle();
    println!("\n=== Deck shuffled! ===");
    Ok(())
}
