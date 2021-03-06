use handle_errors::Error;
use std::collections::HashMap;
use warp::{hyper::StatusCode};

use crate::store::Store;
use crate::types::question::*;
use crate::types::pagination::extract_pagination;



pub async fn get_questions(params: HashMap<String, String>, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))

    } else {
        let res: Vec<Question> = store.questions.read().values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
    // convert to warp json reply
}

// post
pub async fn add_question(store: Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> {
    store.questions.write().insert(question.clone().id, question);

    Ok(warp::reply::with_status(
        "Question added",
        StatusCode::OK
    ))
}

// put
pub async fn update_question(id: String, store: Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound))
    }

    Ok(warp::reply::with_status(
        "Question updated", 
        StatusCode::OK))
}

// delete
pub async fn delete_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status(
            "Question delete", 
            StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound))
    }
}