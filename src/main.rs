mod cli;

use crate::cli::{Commands, DoArgs, LoginArgs, WPEpubCli};
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::EnvFilter;
use wp_mini_epub::{download_story_to_file, login, logout, StoryField};

// --- Application Entry Point ---
#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize the tracing subscriber with EnvFilter.
    // This will default to showing INFO logs for your crate (`wp_epub_cli`)
    // and WARN for all other crates.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("wp_epub_cli=info,warn"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    // 2. Create the reqwest client ONCE and pass it down. No more globals!
    let http_client = Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36")
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

    let story_fields = vec![StoryField::Title];

    let story_response = download_story_to_file(
        client,
        args.id,
        args.include_images,
        args.semaphore as usize,
        &output_dir,
        Some(&story_fields),
    )
    .await?;

    let story_title = story_response.metadata.title.unwrap_or("Unknown Title".to_string());

    info!(title = %story_title, "Story processing completed successfully");

    Ok(())
}

async fn handle_login_command(client: &Client, args: LoginArgs) -> Result<()> {
    info!(username = %args.username, "Handling 'login' command");
    login(client, &args.username, &args.password).await?;
    info!("Login successful!");
    Ok(())
}

async fn handle_logout_command(client: &Client) -> Result<()> {
    info!("Handling 'logout' command");
    logout(client).await?;
    info!("Logout successful!");
    Ok(())
}
