pub async fn get(db_pool: sqlx::PgPool, article_id: uuid::Uuid) -> tide::Result<Vec<String>> {
    #[derive(serde::Deserialize)]
    struct TagNamesView {
        name: String,
    }
    let rows = sqlx::query_as!(
        TagNamesView,
        r#"
        select name from tag
        where article_id=$1;
        "#,
        article_id
    )
    .fetch_all(&db_pool)
    .await?;

    Ok(rows.into_iter().map(|row| row.name).collect())
}

pub async fn create(db_pool: sqlx::PgPool, tag_list: Vec<String>, article_id: uuid::Uuid) -> tide::Result<()> {
    let sql_string = format!(
        r#"
        insert into tag
        (name, article_id)
        values
        "#
    );

    let insert_values = tag_list.into_iter().map(|name| format!("('{}', '{}')", name, article_id.to_string())).collect::<Vec<String>>();

    let sql_string = format!("{} {};", sql_string, insert_values.join(","));

    match sqlx::query(&sql_string).execute(&db_pool).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
    }
}

pub async fn delete(db_pool: sqlx::PgPool, article_id: uuid::Uuid) -> tide::Result<()> {
    match sqlx::query!(
        r#"
        delete from tag
        where article_id=$1
        "#,
        article_id
    ).execute(&db_pool).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
    }
}