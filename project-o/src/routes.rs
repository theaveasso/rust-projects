use warp::Filter;
use serde_json::json;

pub fn bucket_filter() -> impl Filter<Extract= impl warp::Reply, Error= warp::Rejection> + Clone {
    // list items
    warp::get()
        .and(warp::path("v1"))
        .and(warp::path("items"))
        .and(warp::path::end())
        .and_then(bucket_list)
}

async fn bucket_list() -> Result<impl warp::Reply, warp::Rejection> {
    let item = json!([
        {"id": 1, "content": "hello there"},
        {"id": 2, "content": "testing app"}
    ]);
    Ok(warp::reply::json(&item))
}