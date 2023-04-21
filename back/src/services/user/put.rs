use crate::services::{response_ok_and_json, string_to_uuid};

use super::{gen_token, ResAuthUser};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Req {
    pub user: ReqUser,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    user: ResAuthUser,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReqUser {
    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub email: String,

    #[serde(default)]
    pub bio: String,

    #[serde(default)]
    pub password: String,

    #[serde(default)]
    pub image: String,
}

pub async fn handler(mut req: tide::Request<crate::State>) -> tide::Result {
    let Req { user: req_user } = req.body_json::<Req>().await?;
    let payload = req
        .ext::<crate::middlewares::jwt_token::JWTPayload>()
        .unwrap();

    let crate::middlewares::jwt_token::JWTPayload { id, .. } = payload;

    let db_pool = req.state().postgres_pool.clone();

    let crate::applications::user::Entity {
        username,
        image,
        email,
        bio,
        id,
        ..
    } = crate::applications::user::update(db_pool, req_user, string_to_uuid(id)?).await?;

    let token = gen_token(username.clone(), id.clone())?;

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
