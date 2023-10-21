use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::json;
use tokio::task::JoinSet;

#[derive(Deserialize)]
pub struct CardIds {
    pub result: Vec<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Cards {
    pub fields: GermanFields,
    pub note: u64,
}

#[derive(Deserialize, Debug)]
pub struct GermanFields {
    pub Expression: Field,
}

#[derive(Deserialize, Debug)]
pub struct Field {
    pub value: String,
}

#[derive(Deserialize, Debug)]
struct CardInfos {
    result: Vec<Cards>,
}

async fn run<T: DeserializeOwned>(action: &str, params: serde_json::Value) -> Result<T, Error> {
    let client = reqwest::Client::new();
    let body = json!({
        "version": 6,
        "action": action,
        "params": params
    });
    let res = client
        .post("http://127.0.0.1:8765")
        .body(serde_json::to_string(&body).expect("wrong body paramater in post reqest"))
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

pub async fn get_notes() -> Result<Vec<Cards>, Error> {
    let card_ids: CardIds = run(
        "findCards",
        json!({ "query": "deck:language::German is:new note:german_morphman" }),
    )
    .await?;
    let length = card_ids.result.len();
    let mut tasks = vec![];
    let ids_chunked = card_ids.result.chunks(length / 10);
    let mut set = JoinSet::new();
    for ids_chunk in ids_chunked {
        tasks.push(set.spawn(run("cardsInfo", json!({"cards": ids_chunk}))))
    }
    let mut result: Vec<CardInfos> = vec![];
    for _ in tasks {
        while let Some(res) = set.join_next().await {
            if let Ok(Ok(card_infos)) = res {
                result.push(card_infos);
            }
        }
    }
    let result = result.into_iter().flat_map(|ele| ele.result).collect();
    Ok(result)
}
