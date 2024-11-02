mod authentication;
mod friends;
mod login_signup_cli;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    login_signup_cli::login_signup_cli().await;
    Ok(())
}
