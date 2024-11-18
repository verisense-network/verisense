# Verisense Node

> [Verisense](https://verisense.network) node for running AVS inside. 

Verisense is an innovative blockchain project designed to push the boundaries of decentralized application development.

## What makes Verisense different?
Unlike traditional blockchains, Verisense represents a hybrid model that combines the synchronous execution paradigm of traditional blockchains with an asynchronous computation model based on the actor model. This innovative approach significantly enhances network throughput while introducing a new programming paradigm for Web3.

- **Web2-Like Development**: Verisense significantly lowers the barrier to entry for developers by mimicking familiar Web2 patterns, enabling easier and faster adoption.
- **Scalability**: The actor-model and Monadring system allow for the creation of independent, high-performance sub-networks, improving overall scalability.
- **Flexibility**: With capabilities like independent I/O, state management, and timer functions, dApps built on Verisense are more versatile than ever before.
- **Security**: Verisense retains the robust security of PoS consensus while providing modularized control over application-level states.

## Key concepts

### 1. **Consensus Layer**
Verisense features its own PoS-based consensus mechanism, ensuring robust security and decentralization at its core. This serves as the foundation for the execution of decentralized applications and network operations.

### 2. **Nucleus: Actor-Based Application Framework**
The core innovation in Verisense lies in the **Nucleus**, an actor-model application framework that redefines the way decentralized applications are built and executed. Key features of the Nucleus include:
- **Independent Execution**: Each Nucleus operates as an isolated WebAssembly (WASM) module within nodes, ensuring secure and modular application logic.
- **Enhanced I/O Capabilities**: Nuclei can actively handle requests, initiate I/O operations, and interact with external systems, providing unprecedented flexibility for dApps.
- **State Management**: Independent storage spaces for each Nucleus allow for granular state control without interference from other applications.
- **Timers and Scheduling**: Built-in timer functionalities enable advanced scheduling capabilities for dApps.
- **Node Signatures**: Native support for accessing node-level cryptographic operations, such as signing transactions or data.
- **Developer-Friendly Design**: The programming paradigm of the Nucleus closely aligns with Web2 application development practices, making it intuitive for developers transitioning to Web3.

### 3. **Monadring: Lightweight Subnetwork Consensus**
To enable efficient state synchronization for Nucleus instances, Verisense introduces **Monadring**, a lightweight consensus system purpose-built for managing independent subnets within the Verisense network. Monadring allows developers to:

- **Create Independent Subnets**: Build isolated sub-networks within the larger Verisense ecosystem.
- **Efficient State Synchronization**: Achieve high-performance, decentralized state updates tailored for specific applications.
- **Integrate Seamlessly**: Monadring is designed to complement the main consensus layer, ensuring interoperability and security.

For more details, please check our [paper](https://arxiv.org/abs/2408.16094).

### 4. **Restaking**
After Eigenlayer issued the idea, restaking becomes popular. Verisense harnesses the power of restaking, reusing the staking mechanisms of existing tokens to provide decentralized security for its consensus layer instead of bootstrapping a new PoS network from scratch.

Currently we are working with [Karak](https://karak.network/) and [Symbiotic](https://symbiotic.fi/) as security providers.

## Getting start

The tutorial could be found in our [docs](https://docs.verisense.network/developer_guides/basic.html). We are working on bringing more demos, please keep an eye on it.

![avs-overview](https://github.com/user-attachments/assets/7298859d-167e-4221-8274-fcb10ebe68ab)
![cage-inside](https://github.com/user-attachments/assets/8cf5ff91-db6d-437a-b55c-5b77f7bf1b95)
