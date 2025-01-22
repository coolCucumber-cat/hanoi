mod game;
use game::Game;

use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, MethodRouter},
    Router,
};

/// The app state of our program
///
/// `Clone` clones the `Arc<T>`, not the data behind it
///
/// Mutex<T>: share one mutable reference to heap allocated data across threads (panics if Rust's borrowing rules are violated)
/// Arc<T>: share multiple immutable references to data across threads
/// Arc<Mutex<T>>: share multiple mutable references to data across threads
#[derive(Clone)]
struct AppState {
    game: Arc<Mutex<Game>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            game: Arc::new(Mutex::new(Game::new(3))),
        }
    }
}

/// Our routes
mod routes {
    use axum::{extract::State, response::IntoResponse, Json};

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

    pub async fn delete(
        State(AppState { game }): State<AppState>,
        Json(count): Json<usize>,
    ) -> impl IntoResponse {
        let mut game = game.lock().unwrap();
        *game = Game::new(count);
        Json(&*game).into_response()
    }

    pub async fn hint(State(AppState { game }): State<AppState>) -> Json<Move> {
        let game = game.lock().unwrap();
        let hint = game.hint_move();
        Json(hint)
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::get))
        .route(
            "/api/hanoi",
            MethodRouter::new()
                .get(routes::get)
                .post(routes::post)
                .delete(routes::delete),
        )
        .route("/api/hanoi/hint", get(routes::hint))
        .with_state(AppState::new());

    #[cfg(debug_assertions)]
    {
        let mut g = Game::new(4);
        while let Some(()) = g.play() {
            println!("{g:?}");
        }
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// fn main() {
// }
