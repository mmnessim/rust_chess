use crate::chess::game::{Archives, Game, ManyGames};

pub async fn stats(username: &str, client: reqwest::Client) {
    let url = format!("https://api.chess.com/pub/player/{username}/stats");
    if let Ok(response) = client.get(url).send().await {
        println!("{:#?}", response.text().await.unwrap());
    }
}

pub async fn random_game(username: &str, client: reqwest::Client) {
    let url = format!("https://api.chess.com/pub/player/{username}/games/archives");
    if let Ok(response) = client.get(url).send().await {
        let text = response.text().await.unwrap();
        let archives = serde_json::from_str::<Archives>(&text).unwrap().archives;
        let num = rand::random_range(0..archives.len());
        let arch_choice = &archives[num];
        if let Ok(res) = client.get(arch_choice).send().await {
            let inner_text = res.text().await.unwrap();
            let games: Vec<Game> = serde_json::from_str::<ManyGames>(&inner_text)
                    .unwrap()
                    .games;
            let inner_num = rand::random_range(0..games.len());
            println!("{:#?}", &games[inner_num]);
        }
    }
}
