use std::{fs::read_to_string, io, path::Path};

use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TranscriptEntry {
    text: String,
    start: f32,
    duration: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Video {
    id: String,
    thumbnail: String,
    title: String,
    transcription: Vec<TranscriptEntry>,
}

struct MyConfig {
    videos: Vec<Video>,
}

#[macro_use]
extern crate rocket;

fn cli() {
    let json_file_path = Path::new("./data/transcripts.json");

    let file = read_to_string(json_file_path).expect("Could not read file");
    let data: Vec<Video> = serde_json::from_str(&file).unwrap();

    loop {
        let mut search: String = String::new();
        print!("\nPlease enter your search term:\n> ");

        match io::stdin().read_line(&mut search) {
            Ok(_) => {
                println!("Searching for {}", search);
            }
            Err(e) => {
                println!("Something went wrong!");
                println!("{}", e);
            }
        }
        let search = search.trim().to_lowercase();

        if search.len() < 4 {
            println!("Too short search term...");
            continue;
        }

        println!("{search}");

        for v in &data {
            let matching: Vec<&TranscriptEntry> = v
                .transcription
                .iter()
                .filter(|&t| t.text.to_lowercase().contains(&search))
                .collect();

            if matching.len() > 0 {
                println!("\n{}", v.title);
                println!("------------------------------------------------");
                for m in matching {
                    println!("{0} s - '{1}'", m.start, m.text);
                }
            }
        }
    }
}

#[get("/<search>")]
fn search(search: String, state: &State<MyConfig>) -> String {
    if search.len() < 4 {
        return format!("Too short search term...");
    }

    let mut results: Vec<Video> = vec![];

    for v in &state.videos {
        let matching: Vec<TranscriptEntry> = v
            .transcription
            .iter()
            .cloned()
            .filter(|t| t.text.to_lowercase().contains(&search))
            .collect();

        if matching.len() > 0 {
            results.push(Video {
                id: v.id.clone(),
                thumbnail: v.thumbnail.clone(),
                title: v.title.clone(),
                transcription: matching,
            });
        }
    }
    serde_json::to_string(&results).unwrap()
}

#[rocket::main]
async fn main() {
    let mut input: String = String::new();
    print!("\nDo you want to launch the CLI or Server (c/s):\n>");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("'{}'", input.trim().to_lowercase());
            if input.trim().to_lowercase() == "s" {
                let json_file_path = Path::new("./data/transcripts.json");

                let file = read_to_string(json_file_path).expect("Could not read file");
                let data: Vec<Video> = serde_json::from_str(&file).unwrap();
                let config = MyConfig { videos: data };

                let _rocket = rocket::build()
                    .mount("/search", routes![search])
                    .manage(config)
                    .launch()
                    .await;
            } else {
                cli();
            }
        }
        Err(e) => {
            println!("Something went wrong!");
            println!("{}", e);
        }
    }
}
