mod game;
use axum::{routing::get, Router};
use fmt2::write_to::ToString;
use game::Game;

#[derive(Clone)]
struct AppState {
    game: game::Game,
}
//
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

    for _ in 0..20 {
        let hint = g.hint();
        let play = g.play(hint);
        println!("{hint:?} => {play:?}");
    }
}
