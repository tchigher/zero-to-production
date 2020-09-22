use anyhow::Context;
use chapter04::configuration::get_configuration;
use chapter04::startup::run;
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use env_logger::Env;

#[actix_rt::main]
async fn main() -> Result<(), anyhow::Error> {
    // `init` does call `log::set_logger`, so this is all we need to do
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .map_err(anyhow::Error::from)
        .with_context(|| "Failed to connect to Postgres.")?;

    // Here we choose to bind explicitly to localhost, 127.0.0.1, for security
    // reasons. This binding may cause issues in some environments. For example,
    // it causes connectivity issues running in WSL2, where you cannot reach the
    // server when it is bound to WSL2's localhost interface. As a workaround,
    // you can choose to bind to all interfaces, 0.0.0.0, instead, but be aware
    // of the security implications when you expose the server on all interfaces.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
