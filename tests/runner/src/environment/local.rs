use async_trait::async_trait;

use fluvio_system_util::bin::get_fluvio;

use crate::TestOption;

use super::EnvironmentDriver;
use fluvio_command::CommandExt;

/// Local Env driver where we should SPU locally
pub struct LocalEnvDriver {
    option: TestOption,
}

impl LocalEnvDriver {
    pub fn new(option: TestOption) -> Self {
        Self { option }
    }
}

#[async_trait]
impl EnvironmentDriver for LocalEnvDriver {
    /// remove cluster
    async fn remove_cluster(&self) {
        get_fluvio()
            .expect("fluvio not found")
            .arg("cluster")
            .arg("delete")
            .arg("--local")
            .inherit()
            .result()
            .expect("fluvio cluster delete --local should work");
    }

    async fn start_cluster(&self) {
        let mut cmd = get_fluvio().expect("fluvio not found");

        cmd.arg("cluster")
            .arg("start")
            .arg("--spu")
            .arg(self.option.spu.to_string())
            .arg("--local");

        if let Some(log) = &self.option.server_log {
            cmd.arg("--rust-log").arg(log);
        }

        if let Some(log) = &self.option.log_dir {
            cmd.arg("--log-dir").arg(log);
        }

        if self.option.tls() {
            self.set_tls(&self.option, &mut cmd);
        }

        cmd.inherit()
            .result()
            .expect("fluvio cluster start --local should work");
    }
}
