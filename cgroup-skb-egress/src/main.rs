mod main;

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

    Ok(())
}

async fn listing() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;

    // Here we replace heavy type k8s_openapi::api::core::v1::PodSpec with
    #[derive(Clone, Deserialize, Debug)]
    struct PodSpecSimple {
        containers: Vec<ContainerSimple>,
    }
    #[derive(Clone, Deserialize, Debug)]
    struct ContainerSimple {
        #[allow(dead_code)]
        image: String,
    }
    type PodSimple = Object<PodSpecSimple, NotUsed>;

    // Here we simply steal the type info from k8s_openapi, but we could create this from scratch.
    let ar = ApiResource::erase::<k8s_openapi::api::core::v1::Pod>(&());

    let pods: Api<PodSimple> = Api::default_namespaced_with(client, &ar);
    for p in pods.list(&Default::default()).await? {
        log::info!("Pod {} runs: {:?}", p.name_any(), p.spec.containers);
    }

    Ok(())
}

fn containersID() -> anyhow::Result<()> {

    let pods: Api<Pod> = Api::default_namespaced(client);

    let pod = pods.get("blog").await?;
    println!("Got pod: {pod:?}");

    let patch = json!({"spec": {
    "activeDeadlineSeconds": 5
}});
    let pp = PatchParams::apply("kube");
    let patched = pods.patch("blog", &pp, &Patch::Apply(patch)).await?;
    assert_eq!(patched.spec.active_deadline_seconds, Some(5));

    pods.delete("blog", &DeleteParams::default()).await?;

}