#[allow(clippy::module_inception)]
mod cmd;
mod docs;
mod dotf;
mod include;
mod install;
mod list;
mod load;
mod remove;
mod run;
mod sim;
mod synth;
mod update;
mod upgrade;

use anyhow::Result;

pub use crate::cmd::cmd::*;

pub trait Execute {
    async fn execute(&self) -> Result<()>;
}

impl Execute for Cmd {
    async fn execute(&self) -> Result<()> {
        match self {
            Cmd::Upgrade(cmd) => cmd.execute().await,
            Cmd::Include(cmd) => cmd.execute().await,
            Cmd::Update(cmd) => cmd.execute().await,
            Cmd::Remove(cmd) => cmd.execute().await,
            Cmd::Dotf(cmd) => cmd.execute().await,
            Cmd::Install(cmd) => cmd.execute().await,
            Cmd::List(cmd) => cmd.execute().await,
            Cmd::Sim(cmd) => cmd.execute().await,
            Cmd::Docs(cmd) => cmd.execute().await,
            Cmd::Synth(cmd) => cmd.execute().await,
            Cmd::Load(cmd) => cmd.execute().await,
            Cmd::Run(cmd) => cmd.execute().await,
        }
    }
}
