use lingua::Language::{English, German};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    id: u64,
    sentence: String,
}

#[tokio::main]
async fn main() {
    let file = fs::read_to_string("result.json").unwrap();
    let cards: Vec<Entry> = serde_json::from_str(&file).unwrap();
    let cards = Arc::new(cards);
    let mut handles = vec![];
    let (tx, rx) = channel();
    for i in 0..10 {
        let cards = Arc::clone(&cards);
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let languages = vec![English, German];
            let detector: LanguageDetector =
                LanguageDetectorBuilder::from_languages(&languages).build();
            let length = cards.len();
            let mut ids = vec![];
            for z in (i * length) / 10..(i + 1) * length / 10 {
                let detected_language: Option<Language> =
                    detector.detect_language_of(&cards[z].sentence);
                if let Some(English) = detected_language {
                    //                    println!("{:?}", cards[z].sentence);
                    ids.push(cards[z].id)
                }
            }
            tx.send(ids).unwrap();
        });
        handles.push(handle)
    }
    let mut result = vec![];
    for handle in handles {
        let j = rx.recv().unwrap();
        result.extend(j);
        handle.join().unwrap();
    }
    let client = reqwest::Client::new();
    let body = json!({
        "version": 6,
        "action": "deleteNotes",
        "params": json!({
            "notes": result
        })
    });
    println!("{:?}", body);
    let res = client
        .post("http://127.0.0.1:8765")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await;
    println!("{:?}", res);
}
