#[derive(serde::Deserialize)]
struct TagNamesView {
    name: String,
}

pub async fn get(db_pool: &sqlx::PgPool, article_id: uuid::Uuid) -> tide::Result<Vec<String>> {
    let rows = sqlx::query_as!(
        TagNamesView,
        r#"
        select name from tag
        where article_id=$1;
        "#,
        article_id
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows.into_iter().map(|row| row.name).collect())
}

pub async fn create(
    db_pool: &sqlx::PgPool,
    tag_list: &Vec<String>,
    article_id: uuid::Uuid,
) -> tide::Result<()> {
    let sql_string = format!(
        r#"
        insert into tag
        (name, article_id)
        values
        "#
    );

    let insert_values = tag_list
        .into_iter()
        .map(|name| format!("('{}', '{}')", name, article_id.to_string()))
        .collect::<Vec<String>>();

    let sql_string = format!("{} {};", sql_string, insert_values.join(","));

    match sqlx::query(&sql_string).execute(db_pool).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete(db_pool: &sqlx::PgPool, article_id: uuid::Uuid) -> tide::Result<()> {
    match sqlx::query!(
        r#"
        delete from tag
        where article_id=$1
        "#,
        article_id
    )
    .execute(db_pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub async fn list(db_pool: &sqlx::PgPool) -> tide::Result<Vec<String>> {
    let rows = sqlx::query_as!(
        TagNamesView,
        r#"
        with
        follower as (
        	select article_id, id as user_id from favoriting
        	inner join condituser on favoriting.follower_id=condituser.id
        ),
        rank as (
        	select name, count(follower.user_id) as hot from tag
        	left join follower on follower.article_id=tag.article_id
        	group by name
        	order by hot desc
        	limit 10
        )
        select name from rank;
        "#
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows.into_iter().map(|row| row.name).collect())
}
