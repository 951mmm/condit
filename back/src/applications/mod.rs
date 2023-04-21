// ANCHOR dep
use crate::utils::*;

// ANCHOR mod
pub mod user;
pub mod profile;
pub mod article;
pub mod favorite;
pub mod tag;

// ANCHOR utils
fn empty_or_expr(expr_left: &str, expr_right: &String) -> String {
    match be_empty_string(expr_right) {
        true => String::from(""),
        false => format!("{}'{}'",expr_left, expr_right)
    }
}


pub struct Joiner {
    inner: String,
    separetor: &'static str,
    exception: fn(&String) -> bool,
}

impl Joiner {
    pub fn build(separetor: &'static str, exception: fn(&String) -> bool) -> Self {
        return Self {
            inner: String::new(),
            separetor,
            exception,
        };
    }

    pub fn join(&mut self, string: String) -> Self {
        match (self.exception)(&string) {
            true => {}
            false => {
                if !self.inner.is_empty() {
                    self.inner = format!("{}{}{}", self.inner, self.separetor, string);
                }
                else {
                    self.inner = string;
                }
            }
        };
        Self {
            inner: self.inner.clone(),
            separetor: self.separetor,
            exception: self.exception,
        }
    }

    pub fn builder(&self) -> String {
        self.inner.clone()
    }
}