#[derive(Default)]
pub struct Entity {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: String,
}

pub async fn create(
    db_pool: sqlx::PgPool,
    user: crate::services::user::post::ReqUser,
) -> tide::Result<Entity> {
    if cfg!(feature = "debug") {
        Ok(Entity {
            id: uuid::Uuid::new_v4(),
            username: user.username,
            email: user.email,
            password: user.password,
            bio: None,
            image: user.image.unwrap(),
        })
    } else {
        let row = sqlx::query_as!(
            Entity,
            r#"
            INSERT INTO CONDITUSER (USERNAME, EMAIL, PASSWORD, IMAGE) VALUES
            ($1, $2, $3, $4) RETURNING *
            "#,
            user.username,
            user.email,
            user.password,
            user.image,
        )
        .fetch_one(&db_pool)
        .await?;

        Ok(row)
    }
}

pub async fn have_with_email(db_pool: sqlx::PgPool, email: String) -> tide::Result<bool> {
    match sqlx::query!(
        r#"
        select * from condituser where email=$1    
        "#,
        email
    )
    .fetch_optional(&db_pool)
    .await?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn have_with_username(db_pool: sqlx::PgPool, username: String) -> tide::Result<bool> {
    match sqlx::query!(
        r#"
        select * from condituser where username=$1
        "#,
        username
    )
    .fetch_optional(&db_pool)
    .await?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn have(
    db_pool: sqlx::PgPool,
    user: crate::services::user::login::ReqUser,
) -> tide::Result<Option<Entity>> {
    let row = sqlx::query_as!(
        Entity,
        r#"
        SELECT * FROM CONDITUSER WHERE EMAIL=$1 AND PASSWORD=$2
        "#,
        user.email,
        user.password
    )
    .fetch_optional(&db_pool)
    .await?;

    Ok(row)
}

pub async fn get(db_pool: sqlx::PgPool, id: uuid::Uuid) -> tide::Result<Entity> {
    let row = sqlx::query_as!(
        Entity,
        r#"
        SELECT * FROM CONDITUSER WHERE ID=$1
        "#,
        id
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(row)
}

#[cfg(test)]
pub mod tests {
    pub async fn delete(db_pool: sqlx::PgPool, email: String) -> bool {
        match sqlx::query!(r#"DELETE FROM CONDITUSER WHERE EMAIL=$1"#, email)
            .execute(&db_pool)
            .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
