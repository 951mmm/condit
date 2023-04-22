pub mod list {
    use super::super::*;

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Res {
        tags: Vec<String>,
    }
    pub async fn handler(req: tide::Request<crate::State>) -> tide::Result {
        let db_pool = &req.state().postgres_pool;

        let tag_list = crate::applications::tag::list(db_pool).await?;

        response_ok_and_json(Res { tags: tag_list })
    }
}
