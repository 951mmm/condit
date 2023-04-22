use super::super::*;
use super::*;

#[derive(serde::Serialize)]
pub struct Res {
    article: ResArticle,
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let slug = req.param("slug")?;

    let db_pool = &req.state().postgres_pool;

    let article_entity = crate::applications::article::get(db_pool, string_to_uuid(&String::from(slug))?).await?;

    let res_article = get_res_article(article_entity, None, db_pool).await?;

    response_ok_and_json(Res {
        article: res_article
    })
}