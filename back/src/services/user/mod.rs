// ANCHOR dep
use super::*;

// ANCHOR mod
pub mod get;
pub mod login;
pub mod post;
pub mod put;

// ANCHOR pub obj
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ResAuthUser {
    pub username: String,

    pub email: String,

    pub token: String,

    pub bio: Option<String>,

    pub image: String,
}

// ANCHOR utils
fn gen_token(username: String, id: uuid::Uuid) -> tide::Result<String> {
    let jwt_key = std::env::var("JWT_KEY")?;

    let exp = if cfg!(feature = "token_debug") {
        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(1))
            .unwrap_or(chrono::Utc::now())
            .timestamp()
    } else {
        let token_exp_duration = std::env::var("TOKEN_EXP_DURATION")?;

        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(token_exp_duration.parse()?))
            .unwrap_or(chrono::Utc::now())
            .timestamp()
    };

    #[cfg(feature = "token_debug")]
    tide::log::info!("fresh token with exp: {}", exp);

    let jwt_payload = crate::middlewares::jwt_token::JWTPayload {
        username,
        id: id.to_string(),
        exp,
    };
    let token = crate::middlewares::jwt_token::crypt(jwt_key, &jwt_payload)?;

    Ok(token)
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
