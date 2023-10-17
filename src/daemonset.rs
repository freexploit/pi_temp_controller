use std::sync::Arc;

use crate::errors::Error;
use crate::FanSettings;
use kube::api::ObjectMeta;
use k8s_openapi::api::{apps::v1::{DaemonSetSpec, DaemonSet, DaemonSetUpdateStrategy}, core::v1::{PodTemplateSpec, Container, PodSpec}};

