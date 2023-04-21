// ANCHOR dep
use super::*;
use crate::utils::*;

// ANCHOR mod
pub mod feed;
pub mod get;
pub mod list;
pub mod post;
pub mod put;
pub mod delete;

// ANCHOR pub obj
#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct ResArticle {
    pub slug: String,

    pub title: String,

    pub description: String,

    pub body: String,

    // #[serde(rename = "tagList")]
    // pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,

    pub favorited: bool,

    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,

    pub author: crate::services::profile::ResProfile,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReqWrite {
    pub article: ReqWriteArticle,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReqWriteArticle {
    #[serde(default)]
    pub title: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub body: String,

    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
}

// ANCHOR utils
pub async fn get_res_profile(
    payload: Option<&crate::middlewares::jwt_token::JWTPayload>,
    db_pool: sqlx::PgPool,
    author_id: uuid::Uuid,
) -> tide::Result<crate::services::profile::ResProfile> {
    match payload {
        Some(crate::middlewares::jwt_token::JWTPayload {
            id: followee_id, ..
        }) => {
            crate::applications::profile::get_with_id(
                db_pool,
                author_id,
                string_to_uuid(followee_id)?,
            )
            .await
        }
        None => crate::applications::profile::get_with_id_without_auth(db_pool, author_id).await,
    }
}

pub async fn get_favorited(
    payload: Option<&crate::middlewares::jwt_token::JWTPayload>,
    db_pool: sqlx::PgPool,
    author_id: uuid::Uuid,
) -> tide::Result<bool> {
    match payload {
        Some(crate::middlewares::jwt_token::JWTPayload {
            id: follower_id, ..
        }) => {
            crate::applications::favorite::get_favorited(
                db_pool,
                author_id,
                string_to_uuid(follower_id)?,
            )
            .await
        }
        None => Ok(false),
    }
}

pub async fn get_res_article(
    article_entity: crate::applications::article::Entity,
    payload: Option<&crate::middlewares::jwt_token::JWTPayload>,
    db_pool: sqlx::PgPool,
) -> tide::Result<ResArticle> {
    let crate::applications::article::Entity {
        author_id,
        title,
        description,
        body,
        created_at,
        updated_at,
        id,
    } = article_entity;

    let res_profile = get_res_profile(payload, db_pool.clone(), author_id).await?;

    let favorited = get_favorited(payload, db_pool.clone(), author_id).await?;

    let favorites_count =
        crate::applications::favorite::get_favorites_count(db_pool.clone(), id).await?;

    let tag_list = crate::applications::tag::get(db_pool, id).await?;

    let res_article = ResArticle {
        slug: id.to_string(),
        title: title.to_owned(),
        description: description.to_owned(),
        body: body.to_owned(),
        created_at: created_at.to_string(),
        updated_at: updated_at.to_string(),
        author: res_profile,
        tag_list,
        favorited,
        favorites_count,
    };

    Ok(res_article)
}

pub async fn get_res_articles(
    article_entities: Vec<crate::applications::article::Entity>,
    payload: Option<&crate::middlewares::jwt_token::JWTPayload>,
    db_pool: sqlx::PgPool,
) -> tide::Result<Vec<ResArticle>> {
    let mut res_articles = vec![];

    for article_entity in article_entities.into_iter() {
        let res_article = get_res_article(article_entity, payload, db_pool.clone()).await?;

        res_articles.push(res_article);
    }

    Ok(res_articles)
}

pub fn write_error_handler<'a>(
    mut req: tide::Request<crate::State>,
    next: tide::Next<'a, crate::State>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let req_body = req.body_json::<ReqWrite>().await?;

        let ReqWrite {
            article: req_article,
        } = req_body.clone();

        let ReqWriteArticle {
            title,
            description,
            body,
            ..
        } = req_article.clone();

        let res = tide::Response::new(tide::StatusCode::UnprocessableEntity);

        let mut error_body = ErrorBody::default();

        if be_empty_string(&title) {
            error_body.errors.title = wrap_err_str("should not be blank");
            return set_error(res, error_body);
        }

        if be_empty_string(&description) {
            error_body.errors.description = wrap_err_str("should not be blank");
            return set_error(res, error_body);
        }

        if be_empty_string(&body) {
            error_body.errors.body = wrap_err_str("should not be blank");
            return set_error(res, error_body);
        }

        req.set_ext(req_article);
        Ok(next.run(req).await)
    })
}
