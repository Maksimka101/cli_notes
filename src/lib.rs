use std::{error, fmt};

pub mod cli;
pub mod core;

/// Dynamic error
pub type DynError = Box<dyn error::Error>;

/// Implementation of the [DynError].
#[derive(Debug)]
pub struct Failure;

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generic failure happened")
    }
}

impl error::Error for Failure {}
