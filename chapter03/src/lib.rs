use std::str::FromStr;
use std::io::{Error, ErrorKind};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(id: QuestionId, title : String, content: String, tags: Option<Vec<String>>) -> Self {
        Question { id, title, content, tags }
    }
}


#[derive(Debug, Serialize)]
pub struct QuestionId (String);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provide"))
        }
    }
    
}