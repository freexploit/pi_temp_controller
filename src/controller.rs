pub mod errors;
pub mod crds;

use crate::crds::FanSettings;
use anyhow::Result;
use futures::StreamExt;
use kube::{
    api::{Api, PostParams, Resource},
    runtime::controller::{Action, Controller},
    Client,
};
use kube::api::ListParams;
use std::sync::Arc;
use errors::Error;
use tokio::time::Duration;
use tracing::*;

use kube::api::ObjectMeta;
use k8s_openapi::api::{apps::v1::{DaemonSetSpec, DaemonSet, DaemonSetUpdateStrategy}, core::v1::{PodTemplateSpec, Container, PodSpec}};



pub fn daemonset(settings: Arc<FanSettings>) -> Result<DaemonSet, Error> {

    let st = settings.clone();

    let oref = settings.clone().controller_owner_ref(&()).unwrap();


    let podtemplate = PodTemplateSpec {
        metadata: Some(ObjectMeta {
            generate_name: Some(format!("{}-", settings.metadata.name.clone().unwrap())),
            ..ObjectMeta::default()
        }),
        spec: Some(PodSpec{
            containers: vec![Container{
                name: "pi-temp".to_string(),
                args: Some(
                    vec!["-f".to_string(),st.spec.frequency.to_string(),"-m".to_string(),st.spec.max_threshold.to_string() ]
                ),
                ..Default::default()
            }],
            ..Default::default()

        }) 
    };


    let dms = DaemonSetSpec {
        update_strategy: Some(DaemonSetUpdateStrategy{..Default::default()}),
        template: podtemplate,
        ..Default::default()
    }; 

    let dm = DaemonSet {
        metadata: ObjectMeta {
            generate_name: Some(format!("{}-", settings.metadata.name.clone().unwrap())),
            owner_references: Some(vec![oref]),
            ..ObjectMeta::default()
        },
        spec :  Some(dms),
        ..Default::default()
    };

    Ok(dm)
}

/// Controller triggers this whenever our main object or our children changed
async fn reconcile(settings: Arc<FanSettings>, ctx: Arc<Data>) -> Result<Action, Error> {
    let client = &ctx.client;

      //let oref = settings.controller_owner_ref(&()).unwrap();
    let dm = daemonset(settings.clone());

    let dmset_api = Api::<DaemonSet>::namespaced(
        client.clone(),
        settings.clone()
            .metadata
            .namespace
            .as_ref()
            .ok_or(Error::MissingObjectKey(".metadata.namespace"))?,
    );

    let dm_created = dmset_api.create(&PostParams::default(), &dm.unwrap()).await;

    match dm_created {
        Ok(_ds) => Ok(Action::requeue(Duration::from_secs(300))),
        Err(e) => Err(Error::DaemonSetCreationFailed(e))
    }



    //cm_api
        //.patch(
            //cm.metadata
                //.name
                //.as_ref()
                //.ok_or(Error::MissingObjectKey(".metadata.name"))?,
            //&PatchParams::apply("configmapgenerator.kube-rt.nullable.se"),
            //&Patch::Apply(&cm),
        //)
        //.await
        //.map_err(Error::ConfigMapCreationFailed)?;

}

/// The controller triggers this on reconcile errors
fn error_policy(_object: Arc<FanSettings>, error: &Error, _ctx: Arc<Data>) -> Action {
    eprintln!("Reconciliation error:\n{:?}", error);
    Action::requeue(Duration::from_secs(1))
}

// Data we want access to in error/reconcile calls
struct Data {
    client: Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;

    let fan_settings = Api::<FanSettings>::all(client.clone());
    let dms = Api::<DaemonSet>::all(client.clone());

    info!("starting configmapgen-controller");

    Controller::new(fan_settings, ListParams::default())
        .owns(dms, ListParams::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, Arc::new(Data { client }))
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("reconciled {:?}", o),
                Err(e) => warn!("reconcile failed: {}", e),
            }
        })
        .await;
    info!("controller terminated");
    Ok(())
}

