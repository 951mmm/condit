use crate::services::*;

pub fn be_empty_string(str: &String) -> bool {
    str.trim().is_empty()
}

pub fn set_error(mut res: tide::Response, error_body: ErrorBody) -> tide::Result {
    res.set_body(tide::Body::from_json(&error_body)?);
    Ok(res)
}

pub fn wrap_err_str(str: &str) -> Option<Vec<String>> {
    Some(vec![String::from(str)])
}
