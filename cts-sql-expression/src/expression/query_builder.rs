use std::fmt::Display;
use std::fmt::Write;

#[derive(Default)]
pub struct QueryBuilder {
    query: String,
}

impl QueryBuilder {
    pub fn new(init: impl Into<String>) -> Self {
        QueryBuilder{
            query: init.into(),
        }
    }

    pub fn new_select() -> Self{
        QueryBuilder{
            query: String::from("select "),
        }
    }

    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        write!(self.query, "{sql}").expect("error formatting `sql`");
        self
    }

    pub fn build(&mut self) -> String {
        self.query.clone()
    }
}