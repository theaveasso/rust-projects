use warp::Filter;
use colab::{return_error, Store};

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(colab::get_questions)
        .recover(return_error);

    let routes = get_questions;
    
    warp::serve(routes)
        .run(([127,0,0,1], 5000))
        .await;
}

