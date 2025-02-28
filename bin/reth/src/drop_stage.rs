//! Database debugging tool
use crate::{
    dirs::{DbPath, PlatformPath},
    utils::DbTool,
    StageEnum,
};
use clap::Parser;
use reth_db::{
    database::Database,
    mdbx::{Env, WriteMap},
    tables,
    transaction::DbTxMut,
};
use reth_primitives::ChainSpec;
use reth_staged_sync::utils::{chainspec::genesis_value_parser, init::insert_genesis_state};
use reth_stages::stages::EXECUTION;
use std::sync::Arc;
use tracing::info;

/// `reth drop-stage` command
#[derive(Debug, Parser)]
pub struct Command {
    /// The path to the database folder.
    ///
    /// Defaults to the OS-specific data directory:
    ///
    /// - Linux: `$XDG_DATA_HOME/reth/db` or `$HOME/.local/share/reth/db`
    /// - Windows: `{FOLDERID_RoamingAppData}/reth/db`
    /// - macOS: `$HOME/Library/Application Support/reth/db`
    #[arg(global = true, long, value_name = "PATH", verbatim_doc_comment, default_value_t)]
    db: PlatformPath<DbPath>,

    /// The chain this node is running.
    ///
    /// Possible values are either a built-in chain or the path to a chain specification file.
    ///
    /// Built-in chains:
    /// - mainnet
    /// - goerli
    /// - sepolia
    #[arg(
        long,
        value_name = "CHAIN_OR_PATH",
        verbatim_doc_comment,
        default_value = "mainnet",
        value_parser = genesis_value_parser
    )]
    chain: Arc<ChainSpec>,

    stage: StageEnum,
}

impl Command {
    /// Execute `db` command
    pub async fn execute(&self) -> eyre::Result<()> {
        std::fs::create_dir_all(&self.db)?;

        let db = Env::<WriteMap>::open(self.db.as_ref(), reth_db::mdbx::EnvKind::RW)?;

        let tool = DbTool::new(&db)?;

        match &self.stage {
            StageEnum::Execution => {
                tool.db.update(|tx| {
                    tx.clear::<tables::PlainAccountState>()?;
                    tx.clear::<tables::PlainStorageState>()?;
                    tx.clear::<tables::AccountChangeSet>()?;
                    tx.clear::<tables::StorageChangeSet>()?;
                    tx.clear::<tables::Bytecodes>()?;
                    tx.put::<tables::SyncStage>(EXECUTION.0.to_string(), 0)?;
                    insert_genesis_state::<Env<WriteMap>>(tx, self.chain.genesis())?;
                    Ok::<_, eyre::Error>(())
                })??;
            }
            _ => {
                info!("Nothing to do for stage {:?}", self.stage);
            }
        }

        Ok(())
    }
}
