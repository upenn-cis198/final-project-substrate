## Building a Custom Blockchain Application with Substrate
### Background
**Blockchains** are distributed ledgers that help securely verify multiparty transactions.  Blockchains have rapidly grown in relevancy and popularity due to both its transparent and secure nature as well as its popular application as a backbone for crypto currencies such as Bitcoin.  Blockchain applications however hold much more potential than for just cryptocurrencies such as smart contracts and voting. 

A lot of the major blockchain systems today are built in Rust and in fact most of Ethereum clients are written in Rust by Parity Technologies. Parity recently launched [**Substrate**](https://www.parity.io/substrate/), a Rust based package for building custom blockchains.  It takes advantage of the WebAssembly (WASM) virtual machine and has many helpful features for creating new blockchain applications.

### Goal
The goal of our project will be to build a custom blockchain application using Substrate. Our current idea is to build a custom blockchain with Substrate to run economic experiments that are controversial on the Ethereum mainnet. This will also include a bridge to the main Ethereum network (we might abstract away this part) such that users can deposit Ethereum token on the mainnet and mint new token on this experimental network. There are two value propositions for doing this. One, there is now real economic value on this test network to run meaningful experiments. Two, we are now enabling economic agents to move value in between two blockchain networks.

### Learning outcome

Substrate as a sophisticated library written by arguably the best Rust developers in the blockchain ecosystem and it has a relatively steep learning curve. Understanding how low level blockchain systems work is another challenge in and of itself. Through working on this project, we hope to have a better understanding of the blockchain ecosystem and dealing with complex API calls in Substrate to build something useful and meaningful. 

### Team
ZX Zhang, Benjamin Fineran, Thomas O’Keeffe

ZX is the team’s blockchain subject matter expert.
