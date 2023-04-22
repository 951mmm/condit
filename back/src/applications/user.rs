use super::*;
#[derive(Default, sqlx::FromRow)]
pub struct Entity {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: String,
}

pub async fn create(
    db_pool: &sqlx::PgPool,
    user: &crate::services::user::post::ReqUser,
) -> tide::Result<Entity> {
    if cfg!(feature = "debug") {
        Ok(Entity {
            id: uuid::Uuid::new_v4(),
            username: user.username.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
            bio: None,
            image: user.image.clone().unwrap(),
        })
    } else {
        let row = sqlx::query_as!(
            Entity,
            r#"
            INSERT INTO CONDITUSER (USERNAME, EMAIL, PASSWORD, IMAGE) VALUES
            ($1, $2, $3, $4) RETURNING *;
            "#,
            user.username,
            user.email,
            user.password,
            user.image,
        )
        .fetch_one(db_pool)
        .await?;

        Ok(row)
    }
}

pub async fn have_with_email(db_pool: &sqlx::PgPool, email: &String) -> tide::Result<bool> {
    match sqlx::query!(
        r#"
        select * from condituser where email=$1;
        "#,
        email
    )
    .fetch_optional(db_pool)
    .await?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn have_with_username(db_pool: &sqlx::PgPool, username: &String) -> tide::Result<bool> {
    match sqlx::query!(
        r#"
        select * from condituser where username=$1;
        "#,
        username
    )
    .fetch_optional(db_pool)
    .await?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn have(
    db_pool: &sqlx::PgPool,
    user: &crate::services::user::login::ReqUser,
) -> tide::Result<Option<Entity>> {
    let row = sqlx::query_as!(
        Entity,
        r#"
        SELECT * FROM CONDITUSER WHERE EMAIL=$1 AND PASSWORD=$2;
        "#,
        user.email,
        user.password
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(row)
}

pub async fn get(db_pool: &sqlx::PgPool, id: uuid::Uuid) -> tide::Result<Entity> {
    let row = sqlx::query_as!(
        Entity,
        r#"
        select * from condituser where id=$1;
        "#,
        id
    )
    .fetch_one(db_pool)
    .await?;

    Ok(row)
}

pub async fn update(
    db_pool: &sqlx::PgPool,
    req_user: &crate::services::user::put::ReqUser,
    id: uuid::Uuid,
) -> tide::Result<Entity> {
    let crate::services::user::put::ReqUser {
        username,
        bio,
        password,
        image,
        email,
    } = req_user;

    let params = Joiner::build(",", be_empty_string)
        .join(empty_or_expr("bio=", &bio))
        .join(empty_or_expr("password=", &password))
        .join(empty_or_expr("image=", &image))
        .join(empty_or_expr("email=", &email))
        .join(empty_or_expr("username=", &username))
        .builder();

    let sql_string = format!(
        r#"
        update condituser
        set {}
        where id='{}'
        returning *;
        "#,
        params,
        id.to_string()
    );

    let row = sqlx::query_as(sql_string.as_str())
        .fetch_one(db_pool)
        .await?;

    Ok(row)
}

#[cfg(test)]
pub mod tests {
    pub async fn delete(db_pool: &sqlx::PgPool, email: &String) -> bool {
        match sqlx::query!(r#"DELETE FROM CONDITUSER WHERE EMAIL=$1;"#, email)
            .execute(db_pool)
            .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
