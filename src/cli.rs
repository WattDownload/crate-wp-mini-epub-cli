use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wpepub", version, about, long_about = None)]
pub(crate) struct WPEpubCli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Processes a story from a given ID.
    Do(DoArgs),

    /// Login to account
    Login(LoginArgs),

    /// Logout from account
    Logout,
}

/// Represents the arguments for the 'do' subcommand.
#[derive(Args, Debug)]
pub(crate) struct DoArgs {
    /// ID of the story to process
    #[arg(short = 'd', long)]
    pub(crate) id: u64,

    /// Include images in the output PDF
    #[arg(short = 'i', long = "img")]
    pub(crate) include_images: bool,

    /// Output directory path [default: current directory]
    #[arg(short = 'o', long, value_name = "DIRECTORY")]
    pub(crate) output_path: Option<PathBuf>,

    /// Number of chapters to process concurrently
    #[arg(short = 's', long, default_value_t = 20)]
    pub(crate) semaphore: u32,
}

/// Represents the arguments for the 'login' subcommand.
#[derive(Args, Debug)]
pub(crate) struct LoginArgs {
    /// Username for login
    #[arg(short = 'u', long = "username")]
    pub(crate) username: String,

    /// Password for login
    #[arg(short = 'p', long = "password")]
    pub(crate) password: String,
}
