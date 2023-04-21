use super::super::*;
use super::*;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Req {
    pub article: ReqArticle
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Res {
    pub article: ResArticle
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReqArticle {
    pub title: String,

    pub description: String,
    
    pub body: String,

    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
}

pub async fn handler(mut req: tide::Request<crate::State>) -> tide::Result {
    let Req {
        article: req_article
    } = req.body_json::<Req>().await?;

    let ReqArticle {
        tag_list,..
    } = req_article.clone();

    let payload = req.ext::<crate::middlewares::jwt_token::JWTPayload>().unwrap();


    let crate::middlewares::jwt_token::JWTPayload {
        id: author_id, ..
    } = payload;

    let db_pool = req.state().postgres_pool.clone();

    let article_entity = crate::applications::article::create(db_pool.clone(), req_article, string_to_uuid(author_id)?).await?;

    let crate::applications::article::Entity {
        id: article_id, ..
    } = article_entity;
    
    match tag_list {
        Some(tag_list) => crate::applications::tag::create(db_pool.clone(), tag_list, article_id).await?,
        None => {}
    };
    
    let res_article = get_res_article(article_entity, Some(payload), db_pool).await?;

    response_ok_and_json(Res {
        article: res_article
    })
}