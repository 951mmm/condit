use super::super::*;
use super::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    pub article: ResArticle,
}

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let slug = req.param("slug")?;

    let article_id = String::from(slug);

    let req_article = req.ext::<ReqWriteArticle>().unwrap().clone();

    let ReqWriteArticle { tag_list, .. } = req_article.clone();

    let payload = req
        .ext::<crate::middlewares::jwt_token::JWTPayload>()
        .unwrap();

    let db_pool = req.state().postgres_pool.clone();

    let article_id = string_to_uuid(&article_id)?;

    let article_entity = crate::applications::article::update(
        db_pool.clone(),
        req_article,
        article_id,
    )
    .await?;

    // TODO diff update
    crate::applications::tag::delete(db_pool.clone(), article_id).await?;

    match tag_list {
        Some(tag_list) => {
            crate::applications::tag::create(db_pool.clone(), tag_list, article_id).await?;
        }
        None => {}
    };

    let res_article = get_res_article(article_entity, Some(payload), db_pool).await?;

    response_ok_and_json(Res {
        article: res_article,
    })
}
