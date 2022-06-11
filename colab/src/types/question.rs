use serde::{Deserialize, Serialize};
use warp::reject::Reject;

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct QuestionId(pub String);

#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}
