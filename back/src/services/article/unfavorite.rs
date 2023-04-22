use super::*;

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let aritcle_id = req.param("slug").unwrap();

    let article_id = str_to_uuid(aritcle_id)?;

    let crate::middlewares::jwt_token::JWTPayload {
        id: follower_id, ..
    } = req.ext::<crate::middlewares::jwt_token::JWTPayload>().unwrap();

    let follower_id = string_to_uuid(follower_id)?;

    let db_pool = &req.state().postgres_pool;

    crate::applications::favorite::unfavorite(db_pool, article_id, follower_id).await?;

    Ok(tide::StatusCode::Ok.into())
}