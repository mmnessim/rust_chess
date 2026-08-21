use axum::{Json, extract::State};
use reqwest::StatusCode;
use tracing::{error, info};

use crate::{
    chess::{fetch::fetch_random_game_chesscom, game::Game},
    data::{
        db::{get_random_player_from_db, set_inactive},
        player::Player,
    },
    state::AppState,
};

async fn db_test(State(state): State<AppState>) -> Result<Json<Vec<Player>>, StatusCode> {
    tracing::info!("/dbtest GET");
    match sqlx::query_as::<_, Player>("SELECT * FROM players LIMIT 1")
        .fetch_all(&state.pool)
        .await
    {
        Ok(p) => return Ok(Json(p)),
        Err(e) => tracing::error!("Error: {e}"),
    };

    Err(StatusCode::BAD_GATEWAY)
}

pub async fn serve_random_game(State(state): State<AppState>) -> Result<Json<Game>, StatusCode> {
    info!("/random GET");

    const MAX_ATTEMPTS: u32 = 5;

    for attempt in 1..=MAX_ATTEMPTS {
        let player = match get_random_player_from_db(&state.pool).await {
            Ok(p) => p,
            Err(e) => {
                error!("Error fetching player: {e}");
                continue;
            }
        };

        match fetch_random_game_chesscom(&player.username, state.client.clone()).await {
            Ok(g) if g.rules == "chess" => {
                info!("Succeeded after {attempt} attempts");
                return Ok(Json(g));
            }
            Ok(_) => continue, // wrong game type, try again
            Err(e) => {
                error!("Error fetching game for {}: {e}", player.username);
                match set_inactive(&state.pool, &player).await {
                    Ok(_) => continue,
                    Err(e) => {
                        error!("Error updating database {e}");
                        continue;
                    }
                };
            }
        }
    }

    Err(StatusCode::BAD_GATEWAY)
}
