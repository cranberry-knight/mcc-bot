use clap::Clap;

/// Manages operations for service database.
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Options {
    #[clap(subcommand)]
    pub command: Commands,
    
    /// URL to the Postgres SQL server host.
    #[clap(short, long)]
    pub host: String,

    /// Listened port by Postgres SQL server.
    #[clap(short, long)]
    pub port: u16,
}
#[derive(Clap)]
pub enum Commands {
    /// Create initial database.
    Create(CreateOptions),
}
#[derive(Clap)]
pub struct CreateOptions {
    /// A name of created database.
    #[clap(long)]
    pub database_name: String,

    /// A username of Postgress master accont. Password will be interactively requestd.
    #[clap(long, default_value = "postgres")]
    pub master_user_name: String,

    /// A username of created user to own the created database. Password will be interactively requested.
    #[clap(long, default_value = "mcc-bot")]
    pub owner_user_name: String,
}
