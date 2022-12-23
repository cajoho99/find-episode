use std::{fs::read_to_string, io, path::Path};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::State;
use rocket::{Request, Response};
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
#[get("/")]
fn search_default(state: &State<MyConfig>) -> String {
    let mut results: Vec<Video> = vec![];
    for v in &state.videos {
        results.push(Video {
            id: v.id.clone(),
            thumbnail: v.thumbnail.clone(),
            title: v.title.clone(),
            transcription: vec![],
        });
    }
    return serde_json::to_string(&results).unwrap();
}

#[get("/<search>")]
fn search(search: String, state: &State<MyConfig>) -> String {
    let mut results: Vec<Video> = vec![];
    if search.len() == 0 {
        for v in &state.videos {
            results.push(Video {
                id: v.id.clone(),
                thumbnail: v.thumbnail.clone(),
                title: v.title.clone(),
                transcription: vec![],
            })
        }

        return serde_json::to_string(&results).unwrap();
    }

    if search.len() < 4 {
        return format!("Too short search term...");
    }

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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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
                    .mount("/search", routes![search_default, search])
                    .manage(config)
                    .attach(CORS)
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
