use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[kube(group = "tejita.tech", version = "v1", kind = "FanSettings")]
#[kube(shortname = "fstg", namespaced)]
pub struct FanSettingsSpec {
    pub min_threshold: i32 ,
    pub max_threshold: i32 ,
    pub frequency: u64, 
    pub pi: u8 
}
