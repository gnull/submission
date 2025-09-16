use actix_web::{ResponseError};
use derive_more::Display;

#[derive(Display, Debug)]
pub enum SubmError {
    FileNotFound,
    GenericError,
}

impl ResponseError for SubmError {

}
