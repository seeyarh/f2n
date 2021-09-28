use structopt::StructOpt;
use tokio::io::{self, AsyncBufReadExt, BufReader};

/// Simple Stdin to NATS tool
#[derive(Debug, Clone, StructOpt)]
struct Args {
    /// The nats server URLs (separated by comma) (default
    /// "nats://127.0.0.1:4222")
    #[structopt(long, short, default_value = "nats://127.0.0.1:4222")]
    url: String,

    /// User Credentials File
    #[structopt(long = "creds")]
    creds: Option<String>,

    /// Use TLS Secure Connection
    #[structopt(short = "tls")]
    tls: bool,

    /// The subject to use
    #[structopt(short = "subject")]
    subject: String,

    /// The connection name
    #[structopt(short = "name")]
    connection_name: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    let opts = if let Some(creds_path) = args.creds {
        async_nats::Options::with_credentials(creds_path)
    } else {
        async_nats::Options::new()
    };

    let nc = opts
        .with_name(&args.connection_name)
        .tls_required(args.tls)
        .connect(&args.url)
        .await?;

    let subject = args.subject;
    let f = io::stdin();
    let r = BufReader::new(f);
    let mut lines = r.lines();
    while let Some(line) = lines.next_line().await? {
        nc.publish(&subject, line)
            .await
            .expect("failed to publish to NATS");
    }

    Ok(())
}
