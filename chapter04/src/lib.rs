// create a local store for our questions
// using hashmap
use std::{collections::HashMap, str::FromStr, io::Error, io::ErrorKind};

#[derive(Debug, Clone)]
pub struct Store {
    questions: HashMap<QuestionId, Question>
}
impl Store {
    // new: new store object which we can acces and pass around
    pub fn new() -> Self {
        Store { questions: HashMap::new() }
    }

    // init: the store either with a local json file
    // or in our code with a fes example questions
    
    // hard coded example
    pub fn init(&mut self) -> Self {
        let question = Question::new(
            QuestionId::from_str("1").expect("No valid id provided"), 
            "how to ...".to_string(), 
            "need help with ...".to_string(), 
            Some(vec!("general".to_string()))
        );

        self.add_question(question)
    }

    // add_question: adding an larger example base
    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question.clone());
        self
    }
    
    
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId (String);
impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No valid id provided"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>
}
impl Question {
    fn new(id:QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question { id, title, content, tags }
    }
}