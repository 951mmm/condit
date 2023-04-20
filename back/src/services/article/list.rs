use super::super::*;
use super::*;

#[derive(serde::Deserialize, Debug)]
pub struct Req {
    pub tag: Option<String>,
    
    pub author: Option<String>,

    pub favorited: Option<String>,

    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_offset() -> i64 {
    0
}

fn default_limit() -> i64 {
    20
}

// fn get_limit_and_offset(query: &Req) -> (i32, i32) {
//     let Req {
//         limit,
//         offset,
//         ..
//     } = query;

//     return (limit, offset);
// }

pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
    let query: Req = req.query()?;

    let db_pool = req.state().postgres_pool.clone();

    let article_entity = crate::applications::article::list(db_pool, query).await?;


    response_ok_and_json(article_entity)
}
