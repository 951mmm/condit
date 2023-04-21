use std::str::FromStr;

// ANCHOR dep
use serde::Serialize;

// ANCHOR mod
pub mod user;
pub mod profile;
pub mod article;

// ANCHOR pub obj
#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct ErrorBody {
    pub errors: Errors,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct Errors {
    // user
    pub username: Option<Vec<String>>,
    pub email: Option<Vec<String>>,
    pub password: Option<Vec<String>>,
    #[serde(rename = "email or password")]
    pub email_or_password: Option<Vec<String>>,

    // article
    pub title: Option<Vec<String>>,
    pub description: Option<Vec<String>>,
    pub body: Option<Vec<String>>,
}


// ANCHOR utils
pub fn response_ok_and_json<Res>(res_json: Res) -> tide::Result
where Res: Serialize {
    let mut res = tide::Response::new(tide::StatusCode::Ok);

    res.set_body(tide::Body::from_json(&res_json)?);

    Ok(res)
}

pub fn string_to_uuid(string: &String) -> tide::Result<uuid::Uuid> {
    Ok(uuid::Uuid::from_str(string.as_str())?)
}