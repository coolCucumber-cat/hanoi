mod game;
use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use fmt2::write_to::ToString;
use game::Game;

#[derive(Clone)]
struct AppState {
    game: Arc<Mutex<game::Game>>,
}

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         // .route(
//         //     "/api/hanoi",
//         //     get(|AppState { game }| async { game.to_string() }),
//         // )
//         .with_state(AppState {
//             game: game::Game::new(3),
//         });
//
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

fn main() {
    let mut g = Game::new(4);

    while let Some(hint) = g.next() {
        g.play(hint).expect("what the sigma");
        println!("{hint:?} => {g:?}");
    }
}
