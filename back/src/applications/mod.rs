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
        true => String::new(),
        false => format!("{}'{}'",expr_left, expr_right)
    }
}

fn empty_or_statement(statement: &str, val: &String) -> String {
    match be_empty_string(val) {
        true => String::new(),
        false => String::from(statement)
    }
}



/// create sql expr quickly
/// gen sql like:
/// ```sql
/// ...
/// where expr1,expr2,expr3...
/// ```
/// be careful that there are no ',' at begin and end
/// 
/// use joiner to gen
/// ```rust
/// let params = Joiner::build(",", |string| false)
///     .join("expr1".to_string())
///     .join("expr2".to_string())
///     .join("expr3".to_string())
///     .builder();
/// 
/// assert_eq!(params, "expr1,expr2,expr3");
/// ```
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