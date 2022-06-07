use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::{reject::Reject, Rejection, Reply, hyper::StatusCode};

#[derive(Debug, Clone)]
pub struct Store {
    questions: HashMap<QuestionId, Question>
}
impl Store {
    // new
    pub fn new() -> Self {
        Store {
            questions: Self::init()
        }
    }
    // init
    pub fn init() -> HashMap<QuestionId, Question> {
        // read from file
        let file = include_str!("../questions.json");
        serde_json::from_str(file)
            .expect("Fail to read file")
    }
    // add question
    pub fn add_question(mut self, question: &Question) -> Self {
        self.questions.insert(question.id.clone(), question.clone());
        self
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct QuestionId(String);

#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}

// async functions
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(), 
            StatusCode::RANGE_NOT_SATISFIABLE))
    } else {
        Ok(warp::reply::with_status(
            "route not found".to_string(), 
            StatusCode::NOT_FOUND))
    }
}

// GET

// custom error for parameter
#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParams,
}
impl Reject for Error {}
// adding the display trait for error enum
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError(ref err) => write!(f, "cannot parse parameter: {}", err),
            Error::MissingParams => write!(f, "missing parameter")
        }
    }
}

// adding a pagination struct 
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(
            Pagination {
                start: params.get("start").unwrap().parse::<usize>().map_err(Error::ParseError)?,
                end: params.get("end").unwrap().parse::<usize>().map_err(Error::ParseError)?,
            }
        );
    }
    Err(Error::MissingParams)
}

pub async fn get_questions(params: HashMap<String, String>, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    if params.len() > 0 {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))

    } else {
        let res: Vec<Question> = store.questions.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
    // convert to warp json reply
}