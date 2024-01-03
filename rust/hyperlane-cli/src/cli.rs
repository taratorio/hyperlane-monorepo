use clap::{Args, Parser, Subcommand};

/// The Hyperlane CLI supports the following commands:
///     - Send a message: When provided with a origin chain, mailbox address, RPC URL,
///       destination address/chain, and message bytes, the tool should dispatch the message
///       via Hyperlane.
///     - Search for messages: The CLI should allow users to query for messages sent from a
///       specified chain by providing a MatchingList.
#[derive(Parser, Debug)]
#[command(name = "hprln")]
#[command(about = "A CLI to interact with Hyperlane", long_about = None)]
#[command(author, version, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Send a message from an origin chain to destination chain via Hyperlane.
    #[command(author, version, about, long_about = None)]
    Send(SendCmd),

    /// Search for messages sent from a specified chain by providing a Hyperlane MatchingList.
    #[command(author, version, about, long_about = None)]
    Search(SearchCmd),
}

#[derive(Args, Debug)]
pub(crate) struct SendCmd {
    /// Origin chain
    #[arg(short, long, required = true)]
    pub(crate) origin_chain_domain_id: u32,

    /// RPC URL for the origin chain
    #[arg(short = 'r', long, required = true)]
    pub(crate) origin_chain_rpc_url: String,

    /// Mailbox address
    #[arg(short, long, required = true)]
    pub(crate) mailbox_address: String,

    /// Destination chain domain id
    #[arg(short, long, required = true)]
    pub(crate) destination_chain_domain_id: u32,

    /// Destination address on destination chain
    #[arg(short = 'a', long, required = true)]
    pub(crate) destination_address: String,

    /// Message body
    #[arg(short = 'b', long, required = true)]
    pub(crate) message_body_hex: String,

    /// File path to pk
    #[arg(short, long, required = true)]
    pub(crate) pk_file_path: String,
}

#[derive(Args, Debug)]
pub(crate) struct SearchCmd {
    /// Matching list json string
    #[arg(short = 'q', long, required = true)]
    pub(crate) matching_list: String,

    /// RPC URL for the chain being queried
    #[arg(short = 'r', long, required = true)]
    pub(crate) chain_rpc_url: String,

    /// Domain ID of the chain being queried
    #[arg(short = 'd', long, required = true)]
    pub(crate) chain_domain_id: u32,

    /// Mailbox address on the chain being queried
    #[arg(short, long, required = true)]
    pub(crate) mailbox_address: String,

    /// Number of confirmations before we consider a block final
    #[arg(short, long, default_value_t = 1)]
    pub(crate) finality_blocks: u32,

    /// Number of blocks to consider in the search range.
    /// I.e. [latest block - search_range_blocks_width, latest_block]
    #[arg(short, long, default_value_t = 50_000)]
    pub(crate) search_range_blocks_width: u32,
}
