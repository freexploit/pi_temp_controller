use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create ConfigMap: {0}")]
    DaemonSetCreationFailed(#[source] kube::Error),
    #[error("MissingObjectKey: {0}")]
    MissingObjectKey(&'static str),
}
