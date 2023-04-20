use serde::Serialize;

pub mod user;
pub mod profile;
pub mod article;


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JWTPayload {
    pub id: String,
    pub username: String,
    pub exp: i64,
}

pub fn response_ok_and_json<Res>(res_json: Res) -> tide::Result
where Res: Serialize {
    let mut res = tide::Response::new(tide::StatusCode::Ok);

    res.set_body(tide::Body::from_json(&res_json)?);

    Ok(res)
}
