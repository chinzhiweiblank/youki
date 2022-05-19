use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Features{}

#[derive(Deserialize, Serialize, Debug)]
struct FeatureConfig {
    oci_version_min: &'static str,
    oci_version_max: &'static str,
    hooks: Vec<&'static str>,
    mount_options: Vec<&'static str>,
    linux: Linux,
}

#[derive(Deserialize, Serialize, Debug)]
struct Linux {
    namespaces: Vec<String>,
    capabilities: Vec<String>,
    cgroup: Cgroup,
    seccomp: Seccomp,
    apparmor: Apparmor,
    selinux: Selinux,
}

#[derive(Deserialize, Serialize, Debug)]
struct Seccomp {
    enabled: bool,
    actions: Vec<String>,
    operators: Vec<String>,
    archs: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Apparmor {
    enabled: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct Selinux {
    enabled: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct Cgroup {
    v1: bool,
    v2: bool,
    systemd: bool,
    systemduser: bool,
}

pub fn features(_ : Features ) -> Result<()> {
    let feature_config = FeatureConfig {
        oci_version_min: "", // TODO
        oci_version_max: "", // TODO
        hooks: vec!["prestart", "createRuntime", "createContainer", "startContainer", "poststart", "poststop"], // TODO
        mount_options: vec![], // TODO
        linux: Linux {
            namespaces: vec![], // TODO
            capabilities: vec![], // TODO
            cgroup: Cgroup {
                v1: true,
                v2: false,
                systemd: true,
                systemduser: true,
            },
            seccomp: Seccomp {
                enabled: true,
                actions: vec![], // TODO
                operators: vec![], // TODO
                archs: vec![], // TODO
            },
            apparmor: Apparmor {
                enabled: true,
            },
            selinux: Selinux {
                enabled: true,
            },
        },
    };

    println!("{}", serde_json::to_string_pretty(&feature_config).unwrap());
    Ok(())
}