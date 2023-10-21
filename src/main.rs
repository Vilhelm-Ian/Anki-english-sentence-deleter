use lingua::Language::{English, German};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

mod get_cards;
use get_cards::Cards;

#[tokio::main]
async fn main() {
    let notes: Vec<Cards> = match get_cards::get_notes().await {
        Ok(res) => res,
        Err(err) => panic!("problem getting notes {:?}", err),
    };
    println!("scrapped {:?} notes", notes.len());
    //let file = fs::read_to_string("result.json").unwrap();
    //let cards: Vec<Entry> = serde_json::from_str(&file).unwrap();
    let notes = Arc::new(notes);
    let mut handles = vec![];
    let (tx, rx) = channel();
    for i in 0..10 {
        let notes = Arc::clone(&notes);
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let languages = vec![English, German];
            let detector: LanguageDetector =
                LanguageDetectorBuilder::from_languages(&languages).build();
            let length = notes.len();
            let mut ids = vec![];
            for z in (i * length) / 10..(i + 1) * length / 10 {
                let detected_language: Option<Language> =
                    detector.detect_language_of(&notes[z].fields.Expression.value);
                if let Some(English) = detected_language {
                    println!("{:?}", notes[z].fields.Expression.value);
                    ids.push(notes[z].note)
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

    //   let client = reqwest::Client::new();
    //   let body = json!({
    //       "version": 6,
    //       "action": "deleteNotes",
    //       "params": json!({
    //           "notes": result
    //       })
    //   });
    //   let res = client
    //       .post("http://127.0.0.1:8765")
    //       .body(serde_json::to_string(&body).unwrap())
    //       .send()
    //       .await;
    //println!("{:?}", res);
}
