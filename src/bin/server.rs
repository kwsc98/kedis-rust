use tokio::{net::TcpListener, signal};
use structopt::StructOpt;
use kedis_rust::{DEFAULT_PORT, server};

#[tokio::main]
async fn main(){
    //read command args
    let cli = Cli::from_args();
    let port = cli.port.as_deref().unwrap_or(DEFAULT_PORT);
    //bind port
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await.unwrap();
    //start server and monitor shutdown
    server::run(listener, signal::ctrl_c()).await;
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(short,long)]
    port: Option<String>,
}
