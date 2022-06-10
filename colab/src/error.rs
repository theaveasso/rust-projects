pub mod error {
    use warp::{Reply, Rejection, reject::Reject, hyper::StatusCode, body::BodyDeserializeError};

    #[derive(Debug)]
    pub enum Error {
        MissingParams,
        QuestionNotFound,
        ParseError(std::num::ParseIntError),
    }

    impl Reject for Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                Error::MissingParams => write!(f, "Missing parameters"),
                Error::QuestionNotFound => write!(f, "Question not found"),
                Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err)
            }
        }
    }

    pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(error) = r.find::<Error>() {
            Ok(warp::reply::with_status(
                error.to_string(), 
                StatusCode::RANGE_NOT_SATISFIABLE))
        } else if let Some(error) = r.find::<BodyDeserializeError>() {
            Ok(warp::reply::with_status(
                error.to_string(), 
                StatusCode::FORBIDDEN))
        } else {
            Ok(warp::reply::with_status(
                "route not found".to_string(), 
                StatusCode::NOT_FOUND))
        }
    }
}

