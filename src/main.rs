use std::{fs::read_to_string, io, path::Path};

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

fn main() {
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
