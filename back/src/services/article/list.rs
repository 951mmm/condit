use std::str::FromStr;

use super::super::*;
use super::*;

#[derive(serde::Deserialize, Debug)]
pub struct Req {
    // #[serde(default)]
    pub tag: Option<String>,

    // #[serde(default)]
    pub author: Option<String>,

    // #[serde(default)]
    pub favorited: Option<String>,

    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default = "default_offset")]
    pub offset: i64,
}

#[derive(serde::Serialize)]
pub struct Res {
    articles: Vec<ResArticle>,

    #[serde(rename = "articlesCount")]
    articles_count: usize,
}

fn default_offset() -> i64 {
    0
}

fn default_limit() -> i64 {
    20
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let query: Req = req.query()?;

    let db_pool = req.state().postgres_pool.clone();

    let payload = req.ext::<crate::middlewares::jwt_token::JWTPayload>();

    let article_entities = crate::applications::article::list(db_pool.clone(), query).await?;

    let mut res_articles = vec![];

    for article_entity in article_entities.iter() {
        let crate::applications::article::Entity {
            author_id,
            title,
            description,
            body,
            created_at,
            updated_at,
            id,
        } = article_entity;

        let res_profile = match payload {
            Some(crate::middlewares::jwt_token::JWTPayload {
                id: followee_id, ..
            }) => {
                crate::applications::profile::get_with_id(
                    db_pool.clone(),
                    author_id.to_owned(),
                    uuid::Uuid::from_str(followee_id.as_str())?,
                )
                .await?
            }
            None => {
                crate::applications::profile::get_with_id_without_auth(
                    db_pool.clone(),
                    author_id.to_owned(),
                )
                .await?
            }
        };

        let favorited = match payload {
            Some(crate::middlewares::jwt_token::JWTPayload {
                id: follower_id, ..
            }) => {
                crate::applications::article::get_favorited(
                    db_pool.clone(),
                    id.clone(),
                    uuid::Uuid::from_str(follower_id.as_str())?,
                )
                .await?
            }
            None => false,
        };

        let favorites_count =
            crate::applications::article::get_favorites_count(db_pool.clone(), id.to_owned())
                .await?;

        let res_article = ResArticle {
            slug: id.to_string(),
            title: title.to_owned(),
            description: description.to_owned(),
            body: body.to_owned(),
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            author: res_profile,
            favorited,
            favorites_count,
        };

        res_articles.push(res_article);
    }
    let len = res_articles.len();

    response_ok_and_json(Res {
        articles: res_articles,
        articles_count: len,
    })
}
