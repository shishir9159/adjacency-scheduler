use anyhow::Context;
use hyper_util::rt::TokioExecutor;
use kube::{client::ConfigExt, Api, Client, Config, ResourceExt};
use k8s_openapi::api::core::v1::Pod;
use tower::BoxError;
use tracing::*;
use serde::Deserialize;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::infer()
        .await
        .context("Failed to infer Kubernetes configuration")?;

    let https = config.rustls_https_connector()?;
    let service = tower::ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .map_err(BoxError::from)
        .service(hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(https));

    let client = Client::new(service, config.default_namespace);

    let namespace = env::var("KUBE_NAMESPACE").unwrap_or_else(|_| "default".to_string());
    info!("Using namespace: {}", namespace);

    let pods: Api<Pod> = Api::namespaced(client, &namespace);

    match pods.list(&Default::default()).await {
        Ok(pod_list) => {
            for pod in pod_list {
                info!("Pod name: {}", pod.name_any());
            }
        }
        Err(err) => {
            error!("Failed to list pods: {:#?}", err);
        }
    }

    for pod in pods {
        if let Some(pod) = pods.get(pod).await.ok() {
            if let Some(status) = pod.status {
                if let Some(container_statuses) = status.container_statuses {
                    for container in container_statuses {
                        if let Some(container_id) = container.container_id {
                            println!("Container ID: {}", container_id);
                        }
                    }
                }
            }
        } else {
        println!("Pod not found");
        }
    }


    Ok(())
}