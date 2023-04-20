use sqlx::{postgres::PgPoolOptions, PgPool};

mod applications;
mod middlewares;
mod services;

#[derive(Clone)]
pub struct State {
    pub postgres_pool: PgPool,
}

impl State {
    pub async fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("failed to read env 'DATABASE_URL'");
        let max_cons = std::env::var("MAX_CONS").unwrap_or(String::from("10"));
        let pool = PgPoolOptions::new()
            .max_connections(max_cons.parse().expect("env 'MAX_CON' should be integer"))
            .connect(&db_url)
            .await
            .expect("failed to connect the database");
        return State {
            postgres_pool: pool,
        };
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    server().await?;
    return Ok(());
}

async fn server() -> tide::Result<tide::Server<State>> {
    dotenv::dotenv()?;

    let state = State::new().await;

    let mut app = tide::with_state(state.clone());
    #[cfg(not(test))]
    tide::log::start();

    #[cfg(test)]
    tide::log::warn!("test mode");

    #[cfg(feature = "debug")]
    tide::log::warn!("debug mode");

    #[cfg(feature = "test")]
    tide::log::info!("product mode");

    // static resources
    let index_path = std::env::var("INDEX_PATH")?;
    let assets_dir = std::env::var("ASSETS_DIR")?;

    // middlewares
    let jwt_key = std::env::var("JWT_KEY")?;
    let jwt_token_middleware = middlewares::jwt_token::Ware::new(jwt_key)?;

    app.at("/").serve_file(index_path)?;

    app.at("/assets").serve_dir(assets_dir)?;

    // server
    app.at("/api/v1").nest({
        let mut router = tide::with_state(state.clone());

        router
            .at("/users/login")
            .with(services::user::login::error_handler)
            .post(services::user::login::handler);

        router
            .at("/users")
            .with(services::user::post::error_handler)
            .post(services::user::post::handler);

        router
            .at("/user")
            .with(jwt_token_middleware.clone())
            .get(services::user::get::handler);

        router
            .at("/profiles/:username")
            .with(jwt_token_middleware)
            .nest({
                let mut router = tide::with_state(state.clone());

                router.at("/").get(services::profile::get::handler);

                router.at("/follow").post(services::profile::post::handler);

                router
                    .at("/follow")
                    .delete(services::profile::delete::handler);

                router
            });

        router.at("/articles").nest({
            let mut router = tide::with_state(state.clone());

            router.at("/").get(services::article::list::handler);

            router
        });

        router
    });

    #[cfg(not(test))]
    {
        let server_port = std::env::var("SERVER_PORT")?;
        app.clone()
            .listen(format!("localhost:{}", server_port))
            .await?;
    }

    // #[cfg(test)]
    tide::log::info!("test mode");
    return Ok(app);
}
