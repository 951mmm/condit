#[derive(serde::Deserialize, Default)]
pub struct Entity {
    pub id: uuid::Uuid,

    pub created_at: chrono::NaiveDateTime,

    pub updated_at: chrono::NaiveDateTime,

    pub article_id: uuid::Uuid,

    pub user_id: uuid::Uuid,

    pub body: String,
}

pub async fn create(
    db_pool: &sqlx::PgPool,
    article_id: uuid::Uuid,
    user_id: uuid::Uuid,
    body: &String,
) -> tide::Result<Entity> {
    let row = sqlx::query_as_unchecked!(
        Entity,
        r#"
        insert into comment (article_id, user_id, body)
        values ($1, $2, $3)
        returning *;
        "#,
        article_id,
        user_id,
        body
    )
    .fetch_one(db_pool)
    .await?;

    Ok(row)
}

pub async fn list(db_pool: &sqlx::PgPool, article_id: uuid::Uuid) -> tide::Result<Vec<Entity>> {
    let rows = sqlx::query_as_unchecked!(
        Entity,
        r#"
        select * from comment
        where article_id=$1
        order by updated_at desc;
        "#,
        article_id
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}

pub async fn delete(db_pool: &sqlx::PgPool, comment_id: uuid::Uuid) -> tide::Result<()> {
    match sqlx::query!(
        r#"
        delete from comment
        where id=$1;
        "#,
        comment_id
    )
    .execute(db_pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
