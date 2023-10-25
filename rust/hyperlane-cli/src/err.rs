use hyperlane_core::{HyperlaneDomainProtocol, HyperlaneProtocolError};
use url::ParseError;

/// Error types for the Hyperlane CLI
#[derive(thiserror::Error, Debug)]
pub(crate) enum HyperlaneCliError {
    /// Unsupported protocol
    #[error("Unsupported protocol: protocol={0}")]
    UnsupportedProtocol(HyperlaneDomainProtocol),

    /// Transaction execution failure
    #[error("Transaction execution failure: tx_id={0}")]
    TransactionExecutionFailure(String),

    /// HyperlaneProtocolError pass through
    #[error(transparent)]
    HyperlaneProtocolError(#[from] HyperlaneProtocolError),

    /// URL ParseError pass through
    #[error(transparent)]
    UrlParseError(#[from] ParseError),
}
