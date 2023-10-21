const fs = require("fs");

async function run(action, params) {
  try {
    let obj = {
      action,
      params,
      version: 6,
    };
    body = JSON.stringify(obj);
    let res = await fetch("http://127.0.0.1:8765", {
      method: "POST",
      body,
    });
    let data = await res.json();
    return data;
  } catch (err) {
    console.log("error" + err);
  }
}

async function final() {
  let result = [];
  let cards = await run("findCards", { query: "deck:language::German is:new" });
  for (let i = 0; i < cards.result.length; i++) {
    try {
      let card_info = await run("cardsInfo", { cards: [cards.result[i]] });
      result.push({
        id: card_info.result[0].note,
        sentence: card_info.result[0].fields.Expression.value,
      });
      console.log("added card");
      //console.log(card.cardId,card.fields.Expression.value)
    } catch (err) {
      console.log(err);
    }
  }
  fs.writeFile("./result.json", JSON.stringify(result), (err) => {
    if (err) {
      console.error(err);
    }
    // file written successfully
  });
}

final();
