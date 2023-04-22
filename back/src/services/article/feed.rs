
use super::super::*;
use super::*;
#[derive(serde::Serialize)]
pub struct Res {
    articles: Vec<ResArticle>,

    #[serde(rename = "articlesCount")]
    articles_count: usize,
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let payload = req
        .ext::<crate::middlewares::jwt_token::JWTPayload>()
        .unwrap();

    let crate::middlewares::jwt_token::JWTPayload {
        id: follower_id, ..
    } = payload;

    let db_pool = &req.state().postgres_pool;

    let article_entities = crate::applications::article::list_feed(
        db_pool,
        string_to_uuid(follower_id)?,
    )
    .await?;

    let res_articles = get_res_articles(article_entities, Some(payload), db_pool).await?;

    let len = res_articles.len();

    response_ok_and_json(&Res {
        articles: res_articles,
        articles_count: len,
    })
}
