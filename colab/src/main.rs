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
        .and_then(colab::get_questions);

    let add_question = warp::post()
        .and(warp::path("questions")
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(colab::add_question)
    );

    let routes = get_questions.or(add_question).recover(return_error);
    
    warp::serve(routes)
        .run(([127,0,0,1], 5000))
        .await;
}
