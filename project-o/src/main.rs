use serde::{Deserialize, Serialize};
use warp::{Filter, hyper::StatusCode};

use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;

mod routes;

use routes::bucket_filter;

#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
struct ItemId (String);

#[derive(Clone, Deserialize, Serialize)]
struct Item {
    id: ItemId,
    content: String,
    date: String,
    tags: Option<Vec<String>>
}

#[derive(Clone)]
struct Store {
    bucket: Arc<RwLock<HashMap<ItemId, Item>>>
}
impl Store {
    fn new() -> Self {
        Store { bucket: Arc::new(RwLock::new(HashMap::new())) }
    }
}



#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_item = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("items"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(bucket_item);

    let routes = bucket_filter().or(add_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 5000))
        .await;
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}
async fn bucket_item(item: Item, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.bucket.write().insert(item.clone().id, item);

    Ok(warp::reply::with_status(
        "item added", 
        StatusCode::CREATED))
}