use sqlx::{postgres::PgPoolOptions, PgPool};

mod applications;
mod middlewares;
mod services;
mod utils;

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

// fn setup_logger() -> Result<(), fern::InitError> {
//     fern::Dispatch::new()
//         .format(|out, message, record| {
//             out.finish(format_args!(
//                 "{}[{}][{}] {}",
//                 chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
//                 record.target(),
//                 record.level(),
//                 message
//             ))
//         })
//         .level(log::LevelFilter::Info)
//         .chain(std::io::stdout())  // 同时输出到控制台
//         .chain(fern::log_file("output.log")?) // 输出到文件
//         .apply()?;
//     Ok(())
// }
#[async_std::main]
async fn main() -> tide::Result<()> {
    server().await?;
    return Ok(());
}

async fn server() -> tide::Result<tide::Server<State>> {
    dotenv::dotenv()?;

    let state = State::new().await;

    let mut app = tide::with_state(state.clone());
    if !cfg!(test)
    {
        if cfg!(feature = "production") {
            tide::log::with_level(tide::log::LevelFilter::Warn);
        }
        else if cfg!(feature = "debug") {
            tide::log::with_level(tide::log::LevelFilter::Debug);
        }
        else {
            tide::log::start();
        }
        
        
    }

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
    let jwt_token_middleware = middlewares::jwt_token::Ware::new(jwt_key.clone(), false)?;
    let optional_jwt_token_middleware = middlewares::jwt_token::Ware::new(jwt_key, true)?;
    // setup_logger().expect("Logger setup failed");
    // app.with(LogMiddleware::new());

    #[cfg(feature = "production")]
    {
    app.at("/").serve_file(index_path)?;
    app.at("/assets").serve_dir(assets_dir)?;
    }
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
            .at("/user")
            .with(jwt_token_middleware.clone())
            .put(services::user::put::handler);

        router.at("/tags").get(services::tag::list::handler);

        router.at("/profiles/:username").nest({
            let mut router = tide::with_state(state.clone());

            router
                .at("/")
                .with(optional_jwt_token_middleware.clone())
                .get(services::profile::get::handler);

            router
                .at("/follow")
                .with(jwt_token_middleware.clone())
                .post(services::profile::follow::handler);

            router
                .at("/follow")
                .with(jwt_token_middleware.clone())
                .delete(services::profile::unfollow::handler);

            router
        });

        router.at("/articles").nest({
            let mut router = tide::with_state(state.clone());

            router
                .at("/")
                .with(optional_jwt_token_middleware.clone())
                .get(services::article::list::handler);

            router
                .at("/feed")
                .with(jwt_token_middleware.clone())
                .get(services::article::feed::handler);

            router
                .at("/")
                .with(jwt_token_middleware.clone())
                .with(services::article::write_error_handler)
                .post(services::article::post::handler);

            router.at("/:slug").nest({
                let mut router = tide::with_state(state.clone());

                router.at("/").get(services::article::get::handler);

                router
                    .at("/")
                    .with(jwt_token_middleware.clone())
                    .with(services::article::write_error_handler)
                    .put(services::article::put::handler);

                router
                    .at("/")
                    .with(jwt_token_middleware.clone())
                    .delete(services::article::delete::handler);

                router
                    .at("/favorite")
                    .with(jwt_token_middleware.clone())
                    .post(services::article::favorite::handler);

                router
                    .at("/favorite")
                    .with(jwt_token_middleware.clone())
                    .delete(services::article::unfavorite::handler);

                router.at("comments").nest({
                    let mut router = tide::with_state(state.clone());

                    router
                        .at("/")
                        .with(jwt_token_middleware.clone())
                        .with(services::comment::write_error_handler)
                        .post(services::comment::post::handler);

                    router
                        .at("/")
                        .with(optional_jwt_token_middleware.clone())
                        .get(services::comment::list::handler);

                    router
                        .at("/:id")
                        .with(jwt_token_middleware.clone())
                        .delete(services::comment::delete::handler);

                    router
                });

                router
            });

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
