use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CtsError {
    AggregateError(String),
    FieldError(String),
    FilterError(String),
    GroupError(String),
    OrderError(String),
    ParamError(String),
}

impl Display for CtsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CtsError::AggregateError(data) => data,
            CtsError::FieldError(data) => data,
            CtsError::FilterError(data) => data,
            CtsError::GroupError(data) => data,
            CtsError::OrderError(data) => data,
            CtsError::ParamError(data) => data,
        };
        write!(f, "{}", msg)
    }
}