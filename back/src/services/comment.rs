// deps
use super::{profile::ResProfile, *};
use crate::utils::*;

// pub obj
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ReqWrite {
    pub comment: ReqWriteComment,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ReqWriteComment {
    #[serde(default)]
    pub body: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ResComment {
    id: String,

    created_at: String,

    updated_at: String,

    body: String,

    author: ResProfile,
}

// utils
pub async fn get_res_comment(
    comment_entity: crate::applications::comment::Entity,
    payload: Option<&crate::middlewares::jwt_token::JWTPayload>,
    db_pool: &sqlx::PgPool,
) -> tide::Result<ResComment> {
    let crate::applications::comment::Entity {
        id,
        user_id,
        created_at,
        updated_at,
        body,
        ..
    } = comment_entity;

    let res_profile = get_res_profile(payload, db_pool, user_id).await?;

    let res_comment = ResComment {
        id: id.to_string(),
        created_at: created_at.to_string(),
        updated_at: updated_at.to_string(),
        body,
        author: res_profile,
    };

    Ok(res_comment)
}

pub async fn get_res_comments(
    comment_entities: Vec<crate::applications::comment::Entity>,
    payload: Option<&crate::middlewares::jwt_token::JWTPayload>,
    db_pool: &sqlx::PgPool,
) -> tide::Result<Vec<ResComment>> {
    let mut res_comments = vec![];

    for comment_entity in comment_entities.into_iter() {
        let res_comment = get_res_comment(comment_entity, payload, db_pool).await?;

        res_comments.push(res_comment);
    }

    Ok(res_comments)
}

pub fn write_error_handler<'a>(
    mut req: tide::Request<crate::State>,
    next: tide::Next<'a, crate::State>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let ReqWrite { comment } = req.body_json::<ReqWrite>().await?;

        let res = tide::Response::new(tide::StatusCode::UnprocessableEntity);

        let mut error_body = ErrorBody::default();

        if be_empty_string(&comment.body) {
            error_body.errors.body = wrap_err_str("should not be blank");

            return set_error(res, error_body);
        }

        req.set_ext(comment);

        Ok(next.run(req).await)
    })
}

pub mod post {
    use super::*;

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Res {
        pub comment: ResComment,
    }

    pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
        let comment = req.ext::<ReqWriteComment>().unwrap();

        let article_id = req.param("slug").unwrap();

        let article_id = string_to_uuid(&String::from(article_id))?;

        let payload = req
            .ext::<crate::middlewares::jwt_token::JWTPayload>()
            .unwrap();

        let crate::middlewares::jwt_token::JWTPayload { id: user_id, .. } = payload;

        let user_id = string_to_uuid(user_id)?;

        let db_pool = &req.state().postgres_pool;

        let comment_entity =
            crate::applications::comment::create(db_pool, article_id, user_id, &comment.body)
                .await?;

        let res_comment = get_res_comment(comment_entity, Some(payload), db_pool).await?;

        response_ok_and_json(Res {
            comment: res_comment,
        })
    }
}

pub mod list {
    use crate::services::{response_ok_and_json, string_to_uuid};

    use super::{get_res_comments, ResComment};

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Res {
        pub comments: Vec<ResComment>,
    }

    pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
        let article_id = req.param("slug").unwrap();

        let article_id = string_to_uuid(&String::from(article_id))?;

        let payload = req.ext::<crate::middlewares::jwt_token::JWTPayload>();

        let db_pool = &req.state().postgres_pool;

        let comment_entities = crate::applications::comment::list(db_pool, article_id).await?;

        let res_comments = get_res_comments(comment_entities, payload, db_pool).await?;

        response_ok_and_json(Res {
            comments: res_comments,
        })
    }
}

pub mod delete {
    use crate::services::string_to_uuid;

    pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
        let comment_id = req.param("id").unwrap();

        let comment_id = string_to_uuid(&String::from(comment_id))?;

        let db_pool = &req.state().postgres_pool;

        crate::applications::comment::delete(db_pool, comment_id).await?;

        Ok(tide::StatusCode::Ok.into())
    }
}
