use clap::Parser;
use serde_json;
use tonix_unix::UnixStream;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tonic::{Request, Response, Status};

use crate::runtimeV1::{ContainerStatusRequest, RuntimeServiceClient};

pub mod runtimeV1 {
    tonic::include_proto!("runtime.V1");
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Container ID to inspect
    #[clap(short, long)]
    container: String,
}

// Custom connector for Unix domain sockets
fn unix_connector() -> Endpoint {
    Endpoint::try_from("http://[::]:50051") // placeholder scheme
        .unwrap()
        .connect_with_connector_lazy(move || {
            UnixStream::connect("/run/containerd/containerd.sock")
        })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.container.is_empty() {
        eprintln!("Please provide a container ID using the -container flag");
        std::process::exit(1);
    }

    // Create a connection to the gRPC server
    let endpoint = unix_connector();
    let channel = endpoint.connect().await?;
    let mut client = RuntimeServiceClient::new(channel);

    // Create the ContainerStatusRequest
    let request = ContainerStatusRequest {
        container_id: cli.container.clone(),
        verbose: true,
    };

    // Send the request to the gRPC server
    match client.container_status(Request::new(request)).await {
        Ok(response) => {
            let response: Response<_> = response;
            let json_data = serde_json::to_string_pretty(&response.get_ref())?;
            println!("{}", json_data);
        }
        Err(err) => {
            eprintln!("Failed to get container status: {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
