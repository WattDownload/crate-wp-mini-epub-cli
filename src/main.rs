mod cli;

use crate::cli::{Commands, DoArgs, LoginArgs, WPEpubCli};
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use tracing::{info, info_span, instrument, Level};
use wp_mini_epub::{download_and_save_story, login, logout};

// --- Application Entry Point ---
#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize the tracing subscriber. This captures logs and prints them.
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // 2. Create the reqwest client ONCE and pass it down. No more globals!
    let http_client = Client::builder()
        .cookie_store(true)
        .user_agent("Your User Agent Here")
        .build()?;

    // 3. Parse CLI arguments
    let cli = WPEpubCli::parse();
    info!("CLI arguments parsed successfully");

    // 4. Match the subcommand and delegate to handler functions
    match cli.command {
        Commands::Do(args) => handle_do_command(&http_client, args)
            .await
            .context("Failed to process story")?,
        Commands::Login(args) => handle_login_command(&http_client, args)
            .await
            .context("Failed to login")?,
        Commands::Logout => handle_logout_command(&http_client)
            .await
            .context("Failed to logout")?,
    }

    Ok(())
}

async fn handle_do_command(client: &Client, args: DoArgs) -> Result<()> {
    info!(id = args.id, "Handling 'do' command");

    let output_dir = args
        .output_path
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    download_and_save_story(
        client,
        args.id,
        args.include_images,
        args.semaphore as usize,
        &output_dir,
    )
    .await?;

    info!("✅ Story processing completed successfully!");
    Ok(())
}

async fn handle_login_command(client: &Client, args: LoginArgs) -> Result<()> {
    info!(username = %args.username, "Handling 'login' command");
    login(client, &args.username, &args.password).await?;
    info!("✅ Login successful!");
    Ok(())
}

async fn handle_logout_command(client: &Client) -> Result<()> {
    info!("Handling 'logout' command");
    logout(client).await?;
    info!("✅ Logout successful!");
    Ok(())
}
