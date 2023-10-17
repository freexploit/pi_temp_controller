pub mod crds;

use kube::CustomResourceExt;
fn main() {
    print!("{}", serde_yaml::to_string(&crds::FanSettings::crd()).unwrap())
}

