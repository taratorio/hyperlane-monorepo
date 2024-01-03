# Hyperlane CLI

This is a Rust implementation of a CLI tool that can be used to interact with
the [Hyperlane API](https://docs.hyperlane.xyz/docs/introduction/readme). Currently, the CLI supports the following
commands:

1. Send a message: When provided with a origin chain, mailbox address, RPC URL, destination address/chain,
   and message bytes, the tool should dispatch the message via Hyperlane.
2. Search for messages: The CLI should allow users to query for messages sent from a specified chain by providing a
   [MatchingList](https://github.com/hyperlane-xyz/hyperlane-monorepo/blob/main/rust/agents/relayer/src/settings/matching_list.rs).

_Important: only supports EVM chains for now._

## Usage

### Locating the CLI tool directory

```shell
cd rust/hyperlane-cli
```

### Building the CLI tool

```shell
cargo build --release
```

### Locating the CLI tool binary

```shell
cd ../target/release
```

### Manual

```shell
./hyperlane-cli --help
```

```shell
./hyperlane-cli send --help
```

```shell
./hyperlane-cli search --help
```

### Examples

#### Messaging

Sending "Hello World" to Goerli TestRecipient smart contract from Optimism Goerli (following
the [quickstart](https://docs.hyperlane.xyz/docs/build-with-hyperlane/quickstarts/messaging) example for the Messaging
API):

```shell
export RUST_LOG=info

./hyperlane-cli send \
--pk-file-path <CHANGE_ME> \
--origin-chain-domain-id 420 \
--origin-chain-rpc-url https://optimism-goerli.publicnode.com \
--mailbox-address 0xCC737a94FecaeC165AbCf12dED095BB13F037685 \
--destination-chain-domain-id 5 \
--destination-address 0x00000000000000000000000036FdA966CfffF8a9Cdc814f546db0e6378bFef35 \
 --message-body-hex 0x48656c6c6f20576f726c64
```

Example result on
[Hyperlane Explorer](https://explorer.hyperlane.xyz/message/0x8c6de45894e61d81641c2b0bdcadf4af0373c124e88a225f75259243bcebc61d).

_NOTE: make sure to replace the pk-file-path input argument._

#### Queries

NOTE: if querying older blocks we will need to use an archival node and set _--search-range-blocks-width 0_.

```shell
export RUST_LOG=info

./hyperlane-cli search \
--chain-rpc-url https://optimism-goerli.publicnode.com \
--chain-domain-id 420 \
--mailbox-address 0xCC737a94FecaeC165AbCf12dED095BB13F037685 \
--matching-list "[
  {
    \"destinationDomain\": 5,
    \"recipientAddress\": \"0x00000000000000000000000036FdA966CfffF8a9Cdc814f546db0e6378bFef35\"
  }
]"
```
