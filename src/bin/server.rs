use kedis_rust::{server, DB_SIZE, DEFAULT_PORT};
use structopt::StructOpt;
use tokio::{net::TcpListener, signal};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    init_log_helper();
    let cli = Cli::from_args();
    let port = cli.port.as_deref().unwrap_or(DEFAULT_PORT);
    let db_size = cli.db_size.unwrap_or(DB_SIZE);
    //bind port
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    //start server and monitor shutdown
    server::run(listener, signal::ctrl_c(), db_size).await;
}

fn init_log_helper() {
    tracing_subscriber::registry().with(fmt::layer()).init();
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    port: Option<String>,

    #[structopt(short, long)]
    db_size: Option<usize>,
}
