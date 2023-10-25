mod cli;
mod err;
mod mailbox_factory;
mod mailbox_indexer_factory;

use clap::Parser;
use eyre::{Report, Result};
use hyperlane_base::settings::matching_list::MatchingList;
use hyperlane_core::{HyperlaneMessage, Indexer, Mailbox, H256};
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::cli::{Cli, Command, SearchCmd, SendCmd};
use crate::err::HyperlaneCliError;
use crate::mailbox_factory::MailboxFactory;
use crate::mailbox_indexer_factory::MailboxIndexerFactory;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let cli = Cli::parse();
    info!("{cli:?}");

    let res = match cli.command {
        Command::Send(sc) => handle_send_cmd(&sc).await,
        Command::Search(sc) => handle_search_cmd(&sc).await,
    };

    match res {
        Ok(_) => info!("Success: cmd completed!"),
        Err(err) => error!("Failure: cmd failed with {err:?}"),
    }

    Ok(())
}

async fn handle_send_cmd(sc: &SendCmd) -> Result<()> {
    let mailbox = MailboxFactory::new()
        .new_mailbox(
            sc.origin_chain_rpc_url.as_str(),
            sc.origin_chain_domain_id,
            sc.mailbox_address.as_str(),
            sc.pk_file_path.as_str(),
        )
        .await?;

    let msg = HyperlaneMessage {
        destination: sc.destination_chain_domain_id,
        recipient: H256::from_str(sc.destination_address.as_str())?,
        body: sc.message_body_hex.as_bytes().to_vec(),
        ..HyperlaneMessage::default()
    };

    let tx_outcome = mailbox.dispatch(&msg, None).await?;
    let tx_id = tx_outcome.transaction_id.to_string();
    match tx_outcome.executed {
        true => {
            info!("transaction executed successfully: tx_id={tx_id}");
            Ok(())
        }
        false => Err(Report::new(HyperlaneCliError::TransactionExecutionFailure(
            tx_id,
        ))),
    }
}

async fn handle_search_cmd(sc: &SearchCmd) -> Result<()> {
    let indexer = MailboxIndexerFactory::new().new_indexer(
        sc.chain_rpc_url.as_str(),
        sc.chain_domain_id,
        sc.mailbox_address.as_str(),
        sc.finality_blocks,
    )?;

    let matching_list = serde_json::from_str::<MatchingList>(sc.matching_list.as_str()).unwrap();
    let to = indexer.get_finalized_block_number().await?;
    let from = to - sc.search_range_blocks_width;
    let filtered_msgs: Vec<HyperlaneMessage> = indexer
        .fetch_logs(RangeInclusive::new(from, to))
        .await?
        .iter()
        .map(|el| el.0.clone())
        .filter(|el| matching_list.msg_matches(el, false))
        .collect();

    let mut filtered_msgs_info = String::new();
    for (i, m) in filtered_msgs.iter().enumerate() {
        filtered_msgs_info.push_str(format!("  {i} => {m:#?},\n").as_str());
    }

    info!("Filtered messages:[\n{}]", filtered_msgs_info.as_str());
    Ok(())
}
