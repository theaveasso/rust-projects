use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;

use crate::types::question::{Question, QuestionId};

#[derive(Debug, Clone)]
pub struct Store {
    // prevent race conditions
    // Arc : only allows one writer at a time
    // RwLock : allow reader simontaouly
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>
}
impl Store {
    // new
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init()))
        }
    }
    // init
    pub fn init() -> HashMap<QuestionId, Question> {
        // read from file
        let file = include_str!("../questions.json");
        serde_json::from_str(file)
            .expect("Fail to read file")
    }
}