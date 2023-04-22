use super::super::*;
use super::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    pub article: ResArticle,
}

pub async fn handler(mut req: tide::Request<crate::State>) -> tide::Result {
    let req_article = req.ext::<ReqWriteArticle>().unwrap();

    let ReqWriteArticle { tag_list, .. } = req_article;

    let payload = req
        .ext::<crate::middlewares::jwt_token::JWTPayload>()
        .unwrap();

    let crate::middlewares::jwt_token::JWTPayload { id: author_id, .. } = payload;

    let db_pool = &req.state().postgres_pool;

    let article_entity = crate::applications::article::create(
        db_pool,
        req_article,
        string_to_uuid(author_id)?,
    )
    .await?;

    let crate::applications::article::Entity { id: article_id, .. } = article_entity;

    match tag_list {
        Some(tag_list) => {
            crate::applications::tag::create(db_pool, &tag_list, article_id).await?
        }
        None => {}
    };

    let res_article = get_res_article(article_entity, Some(payload), db_pool).await?;

    response_ok_and_json(Res {
        article: res_article,
    })
}
