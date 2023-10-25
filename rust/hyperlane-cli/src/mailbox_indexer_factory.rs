use crate::err::HyperlaneCliError;
use ethers::{prelude::Provider, providers::Http};
use eyre::Report;
use hyperlane_core::{
    utils::hex_or_base58_to_h256, ContractLocator, HyperlaneDomain, HyperlaneDomain::Known,
    HyperlaneDomainProtocol, HyperlaneMessage, Indexer, KnownHyperlaneDomain,
};
use hyperlane_ethereum::EthereumMailboxIndexer;
use std::sync::Arc;

pub struct MailboxIndexerFactory {}

impl MailboxIndexerFactory {
    pub fn new() -> MailboxIndexerFactory {
        MailboxIndexerFactory {}
    }

    pub fn new_indexer(
        self,
        chain_rpc_url: &str,
        chain_domain_id: u32,
        mailbox_address: &str,
        finality_blocks: u32,
    ) -> eyre::Result<Box<dyn Indexer<HyperlaneMessage>>> {
        let chain_domain = Known(KnownHyperlaneDomain::try_from(chain_domain_id)?);
        let protocol = chain_domain.domain_protocol();
        match protocol {
            HyperlaneDomainProtocol::Ethereum => Self::create_ethereum(
                chain_rpc_url,
                &chain_domain,
                mailbox_address,
                finality_blocks,
            ),
            _ => Err(Report::new(HyperlaneCliError::UnsupportedProtocol(
                protocol,
            ))),
        }
    }

    fn create_ethereum(
        chain_rpc_url: &str,
        chain_domain: &HyperlaneDomain,
        mailbox_address: &str,
        finality_blocks: u32,
    ) -> eyre::Result<Box<dyn Indexer<HyperlaneMessage>>> {
        let provider = Arc::new(Provider::<Http>::try_from(chain_rpc_url)?);
        let locator = ContractLocator {
            domain: chain_domain,
            address: hex_or_base58_to_h256(mailbox_address)?,
        };

        Ok(Box::new(EthereumMailboxIndexer::new(
            provider,
            &locator,
            finality_blocks,
        )))
    }
}
