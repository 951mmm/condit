//! get user
//!
//! authenticate token

use std::str::FromStr;

use super::*;

use super::super::*;
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    user: ResAuthUser,
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let payload = req.ext::<crate::middlewares::jwt_token::JWTPayload>().unwrap();

    let crate::middlewares::jwt_token::JWTPayload { id, .. } = payload;

    let db_pool = req.state().postgres_pool.clone();

    let uuid = uuid::Uuid::from_str(id.as_str())?;

    let user = crate::applications::user::get(db_pool, uuid).await?;

    let crate::applications::user::Entity {
        username,
        image,
        email,
        bio,
        ..
    } = user;

    // refresh token
    let token = gen_token(username.clone(), uuid)?;

    response_ok_and_json(Res {
        user: ResAuthUser {
            username,
            image,
            email,
            bio,
            token,
        },
    })
}

#[cfg(test)]
pub mod tests {
    use crate::{State, services::user::ResAuthUser};
    use tide_testing::TideTestingExt;

    use super::Res;



    pub async fn get_token_string(
        app: tide::Server<State>,
    ) -> String {
        let mut res = app.get("/api/v1/user").await.unwrap();
        let Res { user: ResAuthUser { token, ..}} = res.body_json::<Res>().await.unwrap();
        
        token
    }
}
