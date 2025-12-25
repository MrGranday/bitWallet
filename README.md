# bitWallet

A Rust-based, off-chain Bitcoin wallet application with the foundation to be extended into a full-fledged on-chain wallet.

## Current Functionalities

*   **User Management:** Create, find, and delete users.
*   **Authentication:** Basic user login.
*   **Off-Chain Transactions:** Transfer funds between users within the application's own ledger.
*   **Balance Management:** Check user balances.
*   **Transaction History:** View a user's transaction history.
*   **Simulated Deposits/Withdrawals:** Update user balances to simulate deposits and withdrawals.

## Technology Stack

*   **Language:** Rust
*   **Database:** MongoDB
*   **Bitcoin Library:** `bdk` (Bitcoin Development Kit)
*   **Asynchronous Runtime:** `tokio`
*   **HTTP Client:** `reqwest`
*   **Serialization:** `serde`

## Architecture

The project is a monolithic Rust application that connects to a MongoDB database. It has two main components:

1.  **`src/db.rs`:** Handles all database interactions, including user management, off-chain transfers, and transaction logging.
2.  **`bitwallet_chain`:** A separate crate that contains the logic for interacting with the Bitcoin network using the `bdk` library and an Esplora server. This component is not yet integrated with the main application logic.

## Future Work

This project is currently an "off-chain" wallet system. The following steps can be taken to extend it into a full "on-chain" Bitcoin wallet:

1.  **Integrate `bitwallet_chain`:** Connect the `bitwallet_chain` crate with the main application logic.
2.  **On-Chain Deposits:**
    *   Generate a unique Bitcoin address for each user's deposit.
    *   Monitor the blockchain for incoming transactions to that address.
    *   Update the user's balance in the database upon confirmation of a transaction.
3.  **On-Chain Withdrawals:**
    *   Implement a function that takes a user's withdrawal request (destination address and amount).
    *   Use the `bdk` library to create, sign, and broadcast a Bitcoin transaction to the network.
    *   Update the user's balance in the database after the transaction is successfully broadcast.
4.  **Secure User Authentication:**
    *   Implement password hashing using a library like `bcrypt` or `argon2` to securely store user credentials.
5.  **Improve Error Handling:** Enhance error handling to provide more informative feedback to the user.
6.  **API Layer:** Expose the application's functionalities through a REST or gRPC API to be consumed by a frontend application.
7.  **Frontend Application:** Build a web or mobile interface to interact with the wallet.
