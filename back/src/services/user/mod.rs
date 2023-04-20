pub mod get;
pub mod login;
pub mod post;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ResAuthUser {
    pub username: String,

    pub email: String,

    pub token: String,

    pub bio: Option<String>,

    pub image: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct ErrorBody {
    pub errors: Errors,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct Errors {
    pub username: Option<Vec<String>>,
    pub email: Option<Vec<String>>,
    pub password: Option<Vec<String>>,
    #[serde(rename = "email or password")]
    pub email_or_password: Option<Vec<String>>,
}

// utils function
fn gen_token(username: String, id: uuid::Uuid) -> tide::Result<String> {
    let jwt_key = std::env::var("JWT_KEY")?;

    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(10))
        .unwrap_or(chrono::Utc::now())
        .timestamp();

    tide::log::info!("fresh token with exp: {}", exp);

    let jwt_payload = crate::middlewares::jwt_token::JWTPayload {
        username,
        id: id.to_string(),
        exp,
    };
    let token = crate::middlewares::jwt_token::crypt(jwt_key, &jwt_payload)?;

    Ok(token)
}

fn set_error(mut res: tide::Response, error_body: ErrorBody) -> tide::Result {
    res.set_body(tide::Body::from_json(&error_body)?);
    Ok(res)
}

pub fn wrap_err_str(str: &str) -> Option<Vec<String>> {
    Some(vec![String::from(str)])
}

fn be_empty_string(str: &String) -> bool {
    str.trim().is_empty()
}

#[cfg(test)]
pub mod tests {
    use crate::State;
    use serde::Serialize;
    use std::cell::RefCell;
    use std::rc::Rc;
    use tide_testing::TideTestingExt;

    /// call tide::server request api and get response.
    /// wrap with Rc>Refcell impl multiple ref and mutable
    pub async fn request_post<Req>(
        req_body: Req,
        url: &str,
        app: tide::Server<State>,
    ) -> Rc<RefCell<tide_testing::surf::Response>>
    where
        Req: Serialize,
    {
        return Rc::new(RefCell::new(
            app.post(url).body_json(&req_body).unwrap().await.unwrap(),
        ));
    }
}
