use kube::{
    config::{Config, Kubeconfig},
    Client,
};
use std::error::Error;

async fn create_client() -> Result<Client, Box<dyn Error>> {
    // Try to load in-cluster configuration
    let config = match Config::infer().await {
        Ok(cfg) => cfg,
        Err(_) => {
            // Fallback to loading from kubeconfig file
            let kubeconfig_path = Kubeconfig::read_from(
                std::env::var("KUBECONFIG").unwrap_or_else(|_| {
                    dirs::home_dir()
                        .map(|p| p.join(".kube/config"))
                        .expect("Cannot determine home directory")
                        .to_string_lossy()
                        .into_owned()
                }),
            )?;

            // Config::from_kubeconfig(&).await?
        }
    };

    // Create Kubernetes client
    let client = Client::try_from(config)?;

    Ok(client)
}
