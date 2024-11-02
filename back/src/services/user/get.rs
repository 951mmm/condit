//! get user
//!
//! authenticate token

use std::str::FromStr;

use super::super::*;
use super::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    user: ResAuthUser,
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let payload = req
        .ext::<crate::middlewares::jwt_token::JWTPayload>()
        .unwrap();
    // tide::log::info!("get article");
    let crate::middlewares::jwt_token::JWTPayload { id, .. } = payload;

    let db_pool = &req.state().postgres_pool;

    let uuid = uuid::Uuid::from_str(id.as_str())?;

    let user_entity = crate::applications::user::get(db_pool, uuid).await?;

    let res_user = get_res_user(user_entity)?;

    response_ok_and_json(Res { user: res_user })
}

#[cfg(test)]
pub mod tests {
    use crate::{services::user::ResAuthUser, State};
    use tide_testing::TideTestingExt;

    use super::Res;

    #[allow(unused)]
    pub async fn get_token_string(app: tide::Server<State>) -> String {
        let mut res = app.get("/api/v1/user").await.unwrap();
        let Res {
            user: ResAuthUser { token, .. },
        } = res.body_json::<Res>().await.unwrap();

        token
    }
}
