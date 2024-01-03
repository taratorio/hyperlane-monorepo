use ethers::{
    middleware::{NonceManagerMiddleware, SignerMiddleware},
    prelude::Provider,
    providers::{Http, Middleware},
    signers::{LocalWallet, Signer},
};
use eyre::Report;
use hyperlane_core::{
    utils::hex_or_base58_to_h256, ContractLocator, HyperlaneDomain, HyperlaneDomain::Known,
    HyperlaneDomainProtocol, KnownHyperlaneDomain, Mailbox,
};
use hyperlane_ethereum::{BuildableWithProvider, MailboxBuilder, Signers};
use std::fs::read_to_string;

use crate::err::HyperlaneCliError;

pub struct MailboxFactory {}

impl MailboxFactory {
    pub fn new() -> MailboxFactory {
        MailboxFactory {}
    }

    pub async fn new_mailbox(
        self,
        chain_rpc_url: &str,
        chain_domain_id: u32,
        mailbox_address: &str,
        pk_file_path: &str,
    ) -> eyre::Result<Box<dyn Mailbox>> {
        let chain_domain = Known(KnownHyperlaneDomain::try_from(chain_domain_id)?);
        let protocol = chain_domain.domain_protocol();
        match protocol {
            HyperlaneDomainProtocol::Ethereum => {
                Self::create_ethereum(chain_rpc_url, &chain_domain, mailbox_address, pk_file_path)
                    .await
            }
            _ => Err(Report::new(HyperlaneCliError::UnsupportedProtocol(
                protocol,
            ))),
        }
    }

    async fn create_ethereum(
        chain_rpc_url: &str,
        chain_domain: &HyperlaneDomain,
        mailbox_address: &str,
        pk_file_path: &str,
    ) -> eyre::Result<Box<dyn Mailbox>> {
        let provider = Provider::<Http>::try_from(chain_rpc_url)?;
        let signers: Signers = read_to_string(pk_file_path)?
            .trim()
            .parse::<LocalWallet>()?
            .with_chain_id(provider.get_chainid().await?.as_u64())
            .into();

        let address = ethers::prelude::Signer::address(&signers);
        let nonce_provider = NonceManagerMiddleware::new(provider, address);
        let signing_provider = SignerMiddleware::new(nonce_provider, signers);
        let locator = ContractLocator {
            domain: &Known(KnownHyperlaneDomain::try_from(chain_domain.id())?),
            address: hex_or_base58_to_h256(mailbox_address)?,
        };

        Ok(MailboxBuilder {}
            .build_with_provider(signing_provider, &locator)
            .await)
    }
}
