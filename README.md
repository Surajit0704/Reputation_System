# ⭐ Soroban Reputation System

## 📌 Overview

The **Soroban Reputation System** is a decentralized smart contract built on the **Stellar Soroban platform** that enables trust-based scoring for users using their wallet addresses.

This project demonstrates how to build a **lightweight, efficient, and scalable on-chain reputation mechanism** using Soroban smart contracts.

---

## 🎯 Project Description

In decentralized ecosystems, trust is difficult to establish due to the absence of centralized authorities. This project introduces a **reputation layer** that allows users to gain or lose credibility based on interactions.

The system ensures:

* Transparent reputation tracking
* Immutable on-chain storage
* Fast and low-cost execution

---

## ⚙️ What It Does

* Initializes a decentralized reputation storage system
* Assigns reputation scores to wallet addresses
* Allows increasing and decreasing reputation
* Retrieves reputation of any user
* Stores all data securely on-chain

---

## ✨ Key Features

* 🔐 Fully decentralized reputation tracking
* ⚡ Ultra-lightweight smart contract (~1.4 KB WASM)
* ➕ Add reputation points
* ➖ Subtract reputation points
* 🔍 Query user reputation instantly
* 🧩 Easily integratable into Web3 dApps

---

## 🏗️ Smart Contract Functions

| Function         | Description                            |
| ---------------- | -------------------------------------- |
| `init()`         | Initializes reputation storage         |
| `add_rep()`      | Adds reputation points to a user       |
| `subtract_rep()` | Deducts reputation points from a user  |
| `get_rep()`      | Returns the reputation score of a user |

---

## 🛠️ Tech Stack

* **Language:** Rust
* **Framework:** Soroban SDK
* **Blockchain:** Stellar (Soroban Smart Contracts)

---

## 🚀 Build Details

* ✅ Build Status: **Successful**
* 📦 WASM Size: **1402 bytes**
* 🔑 Exported Functions:

  * `add_rep`
  * `get_rep`
  * `init`
  * `subtract_rep`

---

## 🚀 Deployment

### 🔗 Contract Address
![image alt](https://github.com/Surajit0704/Reputation_System/blob/58bbedd66336cee6a88b6762f1f7cbe772aebee2/Screenshot%202026-03-19%20213433.png)

```
CBNVPIUS3AJTM7GVUMPDXHD2UU37MAXVXJAT2B7KUIK2ZYAKYJ575QUN
```

### 🌐 Explorer Links

* 🔍 Stellar Expert:
  https://stellar.expert/explorer/testnet/contract/CBNVPIUS3AJTM7GVUMPDXHD2UU37MAXVXJAT2B7KUIK2ZYAKYJ575QUN

* 🧪 Stellar Lab:
  https://lab.stellar.org/r/testnet/contract/CBNVPIUS3AJTM7GVUMPDXHD2UU37MAXVXJAT2B7KUIK2ZYAKYJ575QUN

---

## 🚀 How to Build

```bash
rustup default stable
rustup target add wasm32v1-none
stellar contract build
```

---

## 🚀 How to Deploy

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source-account alice \
  --network testnet \
  --alias reputation_system
```

---

## 🧪 Example Usage

### Get Reputation

```bash
stellar contract invoke \
  --id CBNVPIUS3AJTM7GVUMPDXHD2UU37MAXVXJAT2B7KUIK2ZYAKYJ575QUN \
  --fn get_rep \
  --arg USER_ADDRESS
```

---

## 💡 Use Cases

* 🧑‍💻 Freelance platforms (client/worker rating systems)
* 🗳️ DAO governance (member credibility scoring)
* 🛒 Marketplace trust systems (seller reputation)
* 🌐 Decentralized social platforms
* 🤝 Peer-to-peer trust layer

---

## 🔮 Future Enhancements

* Role-based access control (admin/moderator)
* Anti-abuse mechanisms
* Reputation decay over time
* Event logging and analytics
* NFT-based reputation badges
* Tokenized reputation system

---

## 📂 Project Structure

```
contract/
│── contracts/
│   └── contract/
│       └── src/
│           └── lib.rs
│── target/
│── Cargo.toml
│── README.md
```

---

## 📜 License

MIT License

---

## 👨‍💻 Author

**Surajit Ghosh**

* GitHub: https://github.com/YOUR_USERNAME
* LinkedIn: https://linkedin.com/in/YOUR_PROFILE

---

## ⭐ Support

If you found this project useful, consider giving it a ⭐ on GitHub!

