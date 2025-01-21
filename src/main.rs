mod game;
use std::sync::{Arc, Mutex};

use axum::{routing::get, Json, Router};
use game::Game;

#[derive(Clone)]
struct AppState {
    game: Arc<Mutex<game::Game>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            game: Arc::new(Mutex::new(game::Game::new(3))),
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/api/hanoi",
            get(|AppState { game }| async {
                // let game = &*game.lock().unwrap();
                // let game = serde_json::to_string(game).unwrap();
                // Json(game)
                // Json("123")
            }),
        )
        .with_state(AppState::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// fn main() {
//     let mut g = Game::new(4);
//
//     while let Some(hint) = g.next() {
//         g.play(hint).expect("what the sigma");
//         println!("{hint:?} => {g:?}");
//     }
// }
