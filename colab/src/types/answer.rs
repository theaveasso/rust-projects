use serde::{Deserialize, Serialize};

use crate::types::question::QuestionId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);