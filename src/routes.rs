use axum::{extract::State, response::IntoResponse, Json};
use axum::extract::Query;
use serde::Deserialize;
use crate::{
    game::{Error, Game, Move},
    AppState,
};

pub async fn get(State(AppState { game }): State<AppState>) -> impl IntoResponse {
    let game = game.lock().unwrap();
    Json(&*game).into_response()
}

pub async fn post(
    State(AppState { game }): State<AppState>,
    Json(player_move): Json<Move>,
) -> Result<impl IntoResponse, Error> {
    let mut game = game.lock().unwrap();
    game.play_with_move(player_move)?;
    Ok(Json(&*game).into_response())
}

#[derive(Deserialize)]
pub struct DeleteQueryParam {
    #[serde(default = "size_default")]
    size: usize,
}

fn size_default() -> usize {
    3
}

pub async fn delete(
    State(AppState { game }): State<AppState>,
    Query(query): Query<DeleteQueryParam>,
) -> impl IntoResponse {
    let mut game = game.lock().unwrap();
    *game = Game::new(query.size);
    Json(&*game).into_response()
}

pub mod hint {
    use super::*;

    pub async fn get(State(AppState { game }): State<AppState>) -> Json<Move> {
        let game = game.lock().unwrap();
        let hint = game.hint_move();
        Json(hint)
    }
}
