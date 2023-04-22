use super::super::*;
use super::*;

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

    let id = string_to_uuid(id)?;

    let db_pool = &req.state().postgres_pool;

    let user_entity = crate::applications::user::update(db_pool, &req_user, id).await?;

    let res_user = get_res_user(user_entity)?;

    response_ok_and_json(Res { user: res_user })
}
