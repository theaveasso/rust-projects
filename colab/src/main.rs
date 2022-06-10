mod types;
mod store;
mod error;
mod routes;

use warp::Filter;

use crate::store::Store;
use crate::routes::question;
use crate::error::error::return_error;



#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    // get
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(question::get_questions);

    // post
    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(question::add_question);

    // put
    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(question::update_question);

    // delete
    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(question::delete_question);

    let routes = get_questions.or(add_question).or(update_question).or(delete_question).recover(return_error);
    
    warp::serve(routes)
        .run(([127,0,0,1], 5000))
        .await;
}
