mod api;
mod cache;
mod server;

use actix_web::{middleware, web, App, HttpServer};
use clap::{Parser, Subcommand};
use serde::de::DeserializeOwned;

use crate::api::{characters, episodes, locations};
use crate::cache::{connect, get, set};
use crate::characters::character::Character;
use crate::episodes::episode::Episode;
use crate::locations::location::Location;
use crate::server::signup;

#[derive(Parser, Debug)]
#[clap(author = "Author Name", version, about)]
struct Arguments {
    // #[clap(short = 'u', long)]
    // username: String,
    #[clap(subcommand)]
    cmd: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    GetCharacters {
        #[clap(forbid_empty_values = true)]
        api_key: String,
        #[clap(forbid_empty_values = false)]
        character: String,
    },
    GetEpisodes {
        #[clap(forbid_empty_values = true)]
        api_key: String,
        #[clap(forbid_empty_values = false)]
        episode: String,
    },
    GetLocations {
        #[clap(forbid_empty_values = true)]
        api_key: String,
        #[clap(forbid_empty_values = false)]
        location: String,
    },
    StartServer {},
}

async fn get_items<T>( key: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    let result = get::<T>(&mut connect(), key).unwrap();
    let s = serde_json::from_str::<Vec<T>>(&result).unwrap();
    return s;
}

async fn populate_cache() -> Option<()> {
    // episodes
    let mut episode_items = Vec::new();
    for i in 1..51 {
        episode_items.push(i)
    }
    let episodes = episodes::episode::get_multiple(episode_items)
        .await
        .unwrap();
    set(&mut connect(), "episodes", episodes);
    println!("cached episodes");

    //characters
    let mut character_items = Vec::new();
    for i in 1..826 {
        character_items.push(i)
    }
    let characters = characters::character::get_multiple(character_items)
        .await
        .unwrap();
    set(&mut connect(), "characters", characters);
    println!("cached characters");

    //locations
    let mut location_items = Vec::new();
    for i in 1..126 {
        location_items.push(i)
    }
    let locations = locations::location::get_multiple(location_items)
        .await
        .unwrap();
    set(&mut connect(), "locations", locations);
    println!("cached locations");

    Some(())
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    match args.cmd {
        SubCommands::GetCharacters {
            character,
            api_key,
        } => {
            if api_key.is_empty() {
                eprintln!("Please enter an API key. One can be created through the signup endpoint")
            }
            if get::<Character>(&mut connect(), &api_key).unwrap().is_empty() {
                eprintln!("User not found. A user could be created by calling the signup endpoint")
            }
            let items: Vec<Character> = get_items(  "characters").await;
            if character.is_empty() {
                println!("{:?}", items)
            } else {
                for i in items.iter() {
                    if i.name == String::from(&character) {
                        println!("{:?}", serde_json::to_string_pretty(&i).unwrap())
                    }
                }
            }
        }
        SubCommands::GetLocations { location, api_key } => {
            if api_key.is_empty() {
                eprintln!("Please enter an API key. One can be created through the signup endpoint")
            }
            if get::<Location>(&mut connect(), &api_key).unwrap().is_empty() {
                eprintln!("User not found. A user could be created by calling the signup endpoint")
            }
            let items: Vec<Location> = get_items(  "locations").await;
            if location.is_empty() {
                println!("{:?}", items)
            } else {
                for i in items.iter() {
                    if i.name == String::from(&location) {
                        println!("{:?}", serde_json::to_string_pretty(&i).unwrap())
                    }
                }
            }
        }
        SubCommands::GetEpisodes { episode, api_key } => {
            if api_key.is_empty() {
                eprintln!("Please enter an API key. One can be created through the signup endpoint")
            }
            if get::<Episode>(&mut connect(), &api_key).unwrap().is_empty() {
                eprintln!("User not found. A user could be created by calling the signup endpoint")
            }
            let items: Vec<Episode> = get_items(  "episodes").await;
            if episode.is_empty() {
                println!("{:?}", items)
            } else {
                for i in items.iter() {
                    if i.name == String::from(&episode) {
                        println!("{:?}", serde_json::to_string_pretty(&i).unwrap())
                    }
                }
            }
        }
        SubCommands::StartServer {} => {
            let server = HttpServer::new(|| {
                App::new()
                    .wrap(middleware::Logger::default())
                    .app_data(web::JsonConfig::default().limit(4096))
                    .route("/signup", web::post().to(signup))
            });
            let _ = populate_cache().await;
            println!("running proxy server on http://localhost:3002....");
            server
                .bind("127.0.0.1:3002")
                .expect("error binding server to address")
                .run()
                .await
                .expect("error running server");
        }
    }
}
