# 🔷 Solana MCP Server Guide by Autumn AIs (LW-MCP)

A part of the **LW-MCP project** under **Autumn AIs**, this server implements a **Model Context Protocol (MCP)** that enables rich, natural language-driven access to **Solana blockchain data** through [Cline](https://marketplace.visualstudio.com/items?itemName=saoudrizwan.claude-dev). It serves as a conversational gateway for developers, analysts, and AI agents to interface with Solana in real-time using 21 core RPC methods.

---

## 🚀 Overview

The Solana MCP Server allows Cline (Claude Dev Tools) and similar intelligent agents to interface with Solana via structured APIs, abstracting away RPC complexities behind a natural language interface.

Whether you're checking balances, inspecting blocks, monitoring staking information, or querying token data, this server empowers seamless access to Solana for both humans and AI models.

---

## 🌐 Supported Solana RPC Methods

The MCP server implements **21 key Solana RPC endpoints**, organized into thematic categories:

### 🧾 Account & Balance Operations
- **`get_sol_balance`** – Retrieve the SOL balance of a wallet address.
- **`get_token_balance`** – Check the balance of a specific SPL token.
- **`get_account_info`** – Fetch account state and data.
- **`get_largest_accounts`** – View the largest SOL holders on the network.

### ⛓ Block & Transaction Data
- **`get_slot`** – Get the latest slot number.
- **`get_block`** – Retrieve block details using slot number.
- **`get_block_time`** – Get the approximate production time of a block.
- **`get_transaction`** – Inspect a transaction by signature.
- **`get_recent_blockhash`** – Fetch a recent blockhash for transaction simulation.

### 🪙 Token Operations
- **`get_token_accounts_by_owner`** – List SPL token accounts for a specific wallet.
- **`get_token_accounts_by_delegate`** – List accounts delegated to a specific key.
- **`get_token_supply`** – Retrieve supply metrics of a token.

### 🛠 Network & System Info
- **`get_epoch_info`** – View epoch, slot, and leader information.
- **`get_version`** – Display the current node version.
- **`get_health`** – Health check of the RPC node.
- **`get_supply`** – Query total supply statistics.
- **`get_inflation_rate`** – Current network inflation rate.
- **`get_cluster_nodes`** – List all cluster nodes and their RPC addresses.
- **`get_minimum_balance_for_rent_exemption`** – Minimum balance required for rent-exemption.

### ⚖️ Staking & Governance
- **`get_vote_accounts`** – Validator voting accounts on-chain.
- **`get_leader_schedule`** – Current validator leader rotation schedule.

---

## ⚙️ Setup with Cline

To integrate the Solana MCP Server with **Cline**:

1. **Add Configuration**  
   Add the following JSON block to your Cline MCP settings file:  
   (Path: `~/Library/Application Support/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json` on macOS)

```json
{
  "mcpServers": {
    "solana": {
      "command": "cargo",
      "args": ["run"],
      "cwd": "/path/to/solana-mcp-server",
      "env": {
        "SOLANA_RPC_URL": "https://api.mainnet-beta.solana.com"
      }
    }
  }
}
```

2. **Restart Cline**  
   After saving the file, restart your VS Code to activate the server integration.

---

## 🧠 Example Natural Language Queries

Once connected, ask Cline:

- “What’s the SOL balance of address `Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr`?”
- “What’s the current Solana epoch and slot?”
- “Get me the latest transaction for wallet X.”
- “How much supply does the USDC token have?”
- “Show vote accounts of current validators.”

---

## 🌱 Environment Variables

- **`SOLANA_RPC_URL`** *(optional)*: The Solana RPC endpoint to connect with.  
  Defaults to: `https://api.mainnet-beta.solana.com`

You can point this to a custom or private RPC node if needed.

---

## 🛠 Development Guide

### 📦 Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/index.html)
- [Solana CLI tools](https://docs.solana.com/cli/install-solana-cli) *(optional for debugging or testing)*

### 🏗 Build the Project
```bash
cargo build
```

### ▶️ Run the Server
```bash
cargo run
```

This starts the Solana MCP server locally. Use `RUST_LOG=debug` for detailed logs.

---

## 🤝 Contributing

We welcome contributions! If you'd like to improve RPC coverage, add caching, or extend protocol support, feel free to open an issue or submit a pull request.

---

## 📄 License

This project is licensed under the [MIT License](./LICENSE).

---

## 🧭 About Autumn AIs & LW-MCP

**Autumn AIs** is a cutting-edge AI infrastructure company focused on building modular, scalable ecosystems for AI agents and autonomous software.  
**LW-MCP (Language-Wrapped Model Context Protocol)** is our standard for enabling natural language-driven interfaces to blockchain, databases, cloud APIs, and more.

This Solana MCP implementation is one of many language-model-ready protocols we’re building for a decentralized and intelligent future.