use std::str::FromStr;

use warp::Filter;

use chapter03::{Question, QuestionId};
#[tokio::main]
async fn main() {
    // create a path filter (router)
    let hi = warp::path("hello").map(|| {
        "Hello, World!"
    });

    let hello = warp::get().map(|| {
        "Hello, There."
    });
    /*
    when passing the route hi to the ::serve() method, warp can accept
    incomming HTTP requests on the given IP address and port, and try to match
    it to the given filters(path)
    */
    warp::serve(hi)
        .run(([127,0,0,1], 8080))
        .await;
        
    warp::serve(hello)
        .run(([127,0,0,1], 1337))
        .await;
}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question {
        id: QuestionId::from_str("1").expect("No id provided"),
        title: "First Question".to_string(),
        content: "Content of question".to_string(),
        tags: Some(vec!("faq".to_string()))
    };

    Ok(warp::reply::json(&question))
}