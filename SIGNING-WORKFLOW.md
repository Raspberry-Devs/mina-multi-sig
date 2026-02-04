# FROST Transaction Signing & Broadcasting Workflow

This guide walks you through signing Mina transactions using FROST threshold signatures and broadcasting them to the network.

## ⚠️ Security Warning

**This repository has not undergone a security audit. It may contain bugs and security vulnerabilities. Use it at your own risk. The authors and contributors take no responsibility for any loss or damage resulting from the use of this code.**

---

## Quick Reference

| Task | Command |
|------|---------|
| Initialize config | `mina-frost-client init -c <CONFIG_PATH>` |
| Export contact | `mina-frost-client export -n <NAME> -c <CONFIG_PATH>` |
| Import contact | `mina-frost-client import <CONTACT_STRING> -c <CONFIG_PATH>` |
| Start DKG (coordinator) | `mina-frost-client dkg -d <DESC> -s <SERVER_URL> -t <THRESHOLD> -S <PUBKEYS> -c <CONFIG_PATH>` |
| Join DKG (participant) | `mina-frost-client dkg -d <DESC> -s <SERVER_URL> -t <THRESHOLD> -c <CONFIG_PATH>` |
| List groups | `mina-frost-client groups -c <CONFIG_PATH>` |
| Coordinate signing | `mina-frost-client coordinator -g <GROUP_PUBKEY> -S <SIGNER_PUBKEYS> -m <TX_FILE> -o <SIG_OUT> -n <NETWORK> -c <CONFIG_PATH>` |
| Join signing | `mina-frost-client participant -g <GROUP_PUBKEY> -c <CONFIG_PATH>` |
| Build GraphQL | `mina-frost-client graphql-build -i <INPUT_JSON> -o <OUTPUT_FILE>` |
| Broadcast | `mina-frost-client graphql-broadcast -g <GRAPHQL_FILE> -e <ENDPOINT_URL>` |

---

## Prerequisites

### Software Requirements

| Software | Version | Install Command |
|----------|---------|-----------------|
| Rust | 1.87.0+ | [TODO: Add platform-specific install notes] |
| mina-frost-client | latest | `cargo install --git https://github.com/Raspberry-Devs/mina-multi-sig.git --locked mina-frost-client` |
| frostd | latest | `cargo install --git https://github.com/ZcashFoundation/frost-zcash-demo.git --locked frostd` |
| mkcert | latest | [TODO: Add platform-specific install notes] |

### Network Requirements

[TODO: Document required network ports, firewall rules, and connectivity requirements between participants and the frostd server]

---

## Section 1: Initial Setup

### 1.1 Install Tools

```bash
# Install mina-frost-client
cargo install --git https://github.com/Raspberry-Devs/mina-multi-sig.git --locked mina-frost-client

# Install frostd server
cargo install --git https://github.com/ZcashFoundation/frost-zcash-demo.git --locked frostd
```

### 1.2 Initialize Configuration

Each participant must initialize their own configuration file:

```bash
mina-frost-client init -c <CONFIG_PATH>
```

**Example:**
```bash
mina-frost-client init -c ~/.frost/alice.toml
```

> **Note:** The config file contains private FROST shares in clear text. Keep it safe and never share it with anyone.

### 1.3 Exchange Contacts

Participants must exchange contact information before DKG or signing sessions.

**Export your contact:**
```bash
mina-frost-client export -n <YOUR_NAME> -c <CONFIG_PATH>
```

**Import another participant's contact:**
```bash
mina-frost-client import <CONTACT_STRING> -c <CONFIG_PATH>
```

**List all contacts:**
```bash
mina-frost-client contacts -c <CONFIG_PATH>
```

[TODO: Document secure methods for exchanging contact strings between participants (e.g., encrypted messaging, in-person exchange, etc.)]

---

## Section 2: Server Setup

The FROST protocol requires a coordination server (`frostd`) for participants to communicate.

### 2.1 Generate TLS Certificates

```bash
mkcert localhost 127.0.0.1 ::1 2>/dev/null
```

This creates `localhost+2.pem` and `localhost+2-key.pem` in the current directory.

### 2.2 Start the Server

```bash
frostd --tls-cert localhost+2.pem --tls-key localhost+2-key.pem
```

The server runs on `localhost:2744` by default.

[TODO: Add guidance for production server deployment, including:
- Running behind a reverse proxy
- Using proper domain certificates
- Server hardening recommendations
- High availability considerations]

For more information, see the [frostd documentation](https://frost.zfnd.org/zcash/server.html).

---

## Section 3: Key Generation

Choose **one** of the following methods to generate group keys.

### Option A: Distributed Key Generation (DKG) — Recommended

DKG distributes key generation across all participants, with no single party ever knowing the complete private key.

**Coordinator** (one participant initiates with `-S` flag listing other participants' public keys):

```bash
mina-frost-client dkg \
  -c <CONFIG_PATH> \
  -d "<GROUP_DESCRIPTION>" \
  -s <SERVER_URL> \
  -t <THRESHOLD> \
  -S <PARTICIPANT_PUBKEY_1>,<PARTICIPANT_PUBKEY_2>
```

**Other Participants** (join without `-S` flag):

```bash
mina-frost-client dkg \
  -c <CONFIG_PATH> \
  -d "<GROUP_DESCRIPTION>" \
  -s <SERVER_URL> \
  -t <THRESHOLD>
```

**Example (2-of-3 threshold):**

```bash
# Alice (coordinator)
mina-frost-client dkg \
  -c ~/.frost/alice.toml \
  -d "Alice, Bob and Eve's group" \
  -s localhost:2744 \
  -t 2 \
  -S <BOB_PUBKEY>,<EVE_PUBKEY>

# Bob (participant)
mina-frost-client dkg \
  -c ~/.frost/bob.toml \
  -d "Alice, Bob and Eve's group" \
  -s localhost:2744 \
  -t 2

# Eve (participant)
mina-frost-client dkg \
  -c ~/.frost/eve.toml \
  -d "Alice, Bob and Eve's group" \
  -s localhost:2744 \
  -t 2
```

> **Important:** All participants must use the **same description** and **threshold** values.

### Option B: Trusted Dealer — Test Only

⚠️ **Warning:** This method is for testing only. The trusted dealer knows all key shares, which defeats the purpose of threshold signatures.

```bash
mina-frost-client trusted-dealer \
  -c <CONFIG_PATH_1> -c <CONFIG_PATH_2> -c <CONFIG_PATH_3> \
  -d "<GROUP_DESCRIPTION>" \
  -N <NAME_1>,<NAME_2>,<NAME_3> \
  -t <THRESHOLD>
```

**Example:**
```bash
mina-frost-client trusted-dealer \
  -c alice.toml -c bob.toml -c eve.toml \
  -d "Test group" \
  -N Alice,Bob,Eve \
  -t 2
```

[TODO: Add additional warnings about why trusted dealer should never be used in production]

### 3.3 Verify Group Creation

After key generation, verify the group was created:

```bash
mina-frost-client groups -c <CONFIG_PATH>
```

Note the **group public key** — you'll need it for signing sessions.

---

## Section 4: Transaction Preparation

Before signing, you need an unsigned transaction in JSON format. This section shows how to generate transactions using o1js.

### 4.1 Project Setup

Create a new Node.js project for transaction generation:

```bash
mkdir mina-tx-generator && cd mina-tx-generator
npm init -y
npm install o1js
npm install -D typescript ts-node @types/node
```

**tsconfig.json:**
```json
{
  "compilerOptions": {
    "target": "es2021",
    "module": "nodenext",
    "moduleResolution": "nodenext",
    "strict": true,
    "esModuleInterop": true,
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true,
    "outDir": "./dist"
  },
  "include": ["src/**/*"]
}
```

Create an output directory:
```bash
mkdir -p tx-json
```

### 4.2 ZKApp Transactions (Primary)

ZKApp transactions are the primary use case for FROST multi-sig on Mina. The key pattern is using `Permissions.signature()` and `this.requireSignature()` to enable signature-based authorization.

#### 4.2.1 State Update Contract

This contract stores on-chain state and requires a signature to modify it:

**src/update_state.ts:**
```typescript
import {
  SmartContract,
  State,
  state,
  method,
  Field,
  Permissions,
  Mina,
  AccountUpdate,
  PublicKey,
} from 'o1js';
import * as fs from 'fs';

// Contract that requires signature to update state
class StateContract extends SmartContract {
  @state(Field) counter = State<Field>();

  init() {
    super.init();
    this.counter.set(Field(0));
    this.account.permissions.set({
      ...Permissions.default(),
      editState: Permissions.signature(),
    });
  }

  @method async incrementCounter() {
    this.requireSignature();  // Requires contract account signature
    const currentValue = this.counter.get();
    this.counter.requireEquals(currentValue);
    this.counter.set(currentValue.add(1));
  }
}

async function generateStateUpdateTx() {
  // Setup local blockchain (proofsEnabled: false for signature-based)
  const Local = await Mina.LocalBlockchain({ proofsEnabled: false });
  Mina.setActiveInstance(Local);

  // Use your FROST group public key as the fee payer
  // Replace with your actual group public key from `mina-frost-client groups`
  const frostGroupPubKey = PublicKey.fromBase58('<GROUP_PUBLIC_KEY>');

  // Contract account (can also be FROST-controlled)
  const contractAccount = Mina.TestPublicKey.random();
  const contract = new StateContract(contractAccount);

  // Compile contract (required for verification key)
  await StateContract.compile();

  // Create deploy transaction
  const deployTx = await Mina.transaction(
    { sender: frostGroupPubKey, fee: 1e8 },
    async () => {
      AccountUpdate.fundNewAccount(frostGroupPubKey);
      await contract.deploy();
    }
  );
  fs.writeFileSync('./tx-json/deploy-state-contract.json', deployTx.toJSON());
  console.log('Deploy transaction saved to ./tx-json/deploy-state-contract.json');

  // Create state update transaction
  const updateTx = await Mina.transaction(
    { sender: frostGroupPubKey, fee: 1e8 },
    async () => {
      await contract.incrementCounter();
    }
  );
  fs.writeFileSync('./tx-json/update-state-transaction.json', updateTx.toJSON());
  console.log('Update transaction saved to ./tx-json/update-state-transaction.json');
}

generateStateUpdateTx();
```

Run with:
```bash
npx ts-node src/update_state.ts
```

#### 4.2.2 Permissions Update Contract

This contract allows updating its own permissions via signature:

**src/update_permissions.ts:**
```typescript
import {
  SmartContract,
  method,
  Permissions,
  Mina,
  AccountUpdate,
  PublicKey,
} from 'o1js';
import * as fs from 'fs';

class PermissionsContract extends SmartContract {
  init() {
    super.init();
    this.account.permissions.set({
      ...Permissions.default(),
      setPermissions: Permissions.signature(),
    });
  }

  @method async updatePermissions() {
    this.requireSignature();
    this.account.permissions.set({
      ...Permissions.default(),
      editState: Permissions.proofOrSignature(),
      send: Permissions.proof(),
      setPermissions: Permissions.signature(),
      setVerificationKey: Permissions.VerificationKey.impossibleDuringCurrentVersion(),
    });
  }
}

async function generatePermissionsUpdateTx() {
  const Local = await Mina.LocalBlockchain({ proofsEnabled: false });
  Mina.setActiveInstance(Local);

  const frostGroupPubKey = PublicKey.fromBase58('<GROUP_PUBLIC_KEY>');
  const contractAccount = Mina.TestPublicKey.random();
  const contract = new PermissionsContract(contractAccount);

  await PermissionsContract.compile();

  // Deploy transaction
  const deployTx = await Mina.transaction(
    { sender: frostGroupPubKey, fee: 1e8 },
    async () => {
      AccountUpdate.fundNewAccount(frostGroupPubKey);
      await contract.deploy();
    }
  );
  fs.writeFileSync('./tx-json/deploy-permissions-contract.json', deployTx.toJSON());

  // Update permissions transaction
  const updateTx = await Mina.transaction(
    { sender: frostGroupPubKey, fee: 1e8 },
    async () => {
      await contract.updatePermissions();
    }
  );
  fs.writeFileSync('./tx-json/update-permissions-transaction.json', updateTx.toJSON());
  console.log('Transactions saved to ./tx-json/');
}

generatePermissionsUpdateTx();
```

#### 4.2.3 Verification Key Update Contract

This contract allows updating its verification key (useful after Mina hard forks):

**src/update_verification_key.ts:**
```typescript
import {
  SmartContract,
  VerificationKey,
  method,
  Permissions,
  Mina,
  AccountUpdate,
  PublicKey,
} from 'o1js';
import * as fs from 'fs';

class UpdatableContract extends SmartContract {
  init() {
    super.init();
    this.account.permissions.set({
      ...Permissions.default(),
      setVerificationKey: Permissions.VerificationKey.signature(),
    });
  }

  @method async updateVerificationKey(verificationKey: VerificationKey) {
    this.requireSignature();
    this.account.verificationKey.set(verificationKey);
  }
}

// New contract version with different verification key
class NewContractVersion extends SmartContract {
  @method async newMethod() {
    // Different method = different verification key
  }
}

async function generateVerificationKeyUpdateTx() {
  const Local = await Mina.LocalBlockchain({ proofsEnabled: false });
  Mina.setActiveInstance(Local);

  const frostGroupPubKey = PublicKey.fromBase58('<GROUP_PUBLIC_KEY>');
  const contractAccount = Mina.TestPublicKey.random();
  const contract = new UpdatableContract(contractAccount);

  // Compile original contract
  await UpdatableContract.compile();

  // Deploy transaction
  const deployTx = await Mina.transaction(
    { sender: frostGroupPubKey, fee: 1e8 },
    async () => {
      AccountUpdate.fundNewAccount(frostGroupPubKey);
      await contract.deploy();
    }
  );
  fs.writeFileSync('./tx-json/deploy-updatable-contract.json', deployTx.toJSON());

  // Compile new contract version for its verification key
  const { verificationKey: newVerificationKey } = await NewContractVersion.compile();

  // Update verification key transaction
  const updateTx = await Mina.transaction(
    { sender: frostGroupPubKey, fee: 1e8 },
    async () => {
      await contract.updateVerificationKey(newVerificationKey);
    }
  );
  fs.writeFileSync('./tx-json/update-verification-key-transaction.json', updateTx.toJSON());
  console.log('Transactions saved to ./tx-json/');
}

generateVerificationKeyUpdateTx();
```

#### Transaction Structure

The generated JSON has this structure:

```json
{
  "feePayer": {
    "body": {
      "publicKey": "<FEE_PAYER_PUBLIC_KEY>",
      "fee": "100000000",
      "nonce": "0",
      "validUntil": null
    },
    "authorization": ""
  },
  "accountUpdates": [
    {
      "body": {
        "publicKey": "<CONTRACT_PUBLIC_KEY>",
        "balanceChange": { "magnitude": "0", "sgn": "Positive" },
        ...
      },
      "authorization": {
        "signature": null
      }
    }
  ],
  "memo": ""
}
```

#### Signature Injection

FROST signatures are injected into the transaction at:

1. **Fee Payer** — `feePayer.authorization` (always required)
2. **Account Updates** — `accountUpdates[*].authorization.signature` (when `requireSignature()` is called)

| Transaction Type | Signatures Required |
|-----------------|---------------------|
| Deploy | Fee payer + Contract account |
| State update | Fee payer + Contract account (if `requireSignature()` used) |
| Permissions update | Fee payer + Contract account (if `requireSignature()` used) |
| Verification key update | Fee payer + Contract account (if `requireSignature()` used) |

#### Nonce Management

Query the current nonce for your FROST group account using GraphQL:

```graphql
query {
  account(publicKey: "<GROUP_PUBLIC_KEY>") {
    nonce
  }
}
```

Or use the Mina CLI:
```bash
mina account get --public-key <GROUP_PUBLIC_KEY>
```

When generating transactions, set the correct nonce:

```typescript
const tx = await Mina.transaction(
  { sender: frostGroupPubKey, fee: 1e8, nonce: 5 },  // Set nonce explicitly
  async () => {
    // ... transaction logic
  }
);
```

### 4.3 Legacy Payment Transactions

For simple payment transactions without zkApp functionality, create a JSON file manually:

**tx-json/payment.json:**
```json
{
  "to": "<RECEIVER_ADDRESS>",
  "from": "<GROUP_PUBLIC_KEY>",
  "fee": "100000000",
  "amount": "1000000000",
  "nonce": "0",
  "memo": "FROST payment",
  "valid_until": "4294967295",
  "tag": [false, false, false]
}
```

| Field | Description |
|-------|-------------|
| `to` | Recipient's Mina address (B62...) |
| `from` | Sender's Mina address (your FROST group public key) |
| `fee` | Transaction fee in nanomina (100000000 = 0.1 MINA) |
| `amount` | Transfer amount in nanomina (1000000000 = 1 MINA) |
| `nonce` | Sender's current account nonce |
| `memo` | Optional memo (max 32 characters) |
| `valid_until` | Slot number until which transaction is valid (max = 4294967295) |
| `tag` | Transaction type flags `[false, false, false]` for payments |

### 4.4 Key Technical Notes

- **`proofsEnabled: false`** — Transactions use signatures instead of zero-knowledge proofs
- **`Permissions.signature()`** — Allows authorization via signature instead of proof
- **`this.requireSignature()`** — Method call that requires contract account signature
- **Compilation is still required** — Generates verification key needed for deployment
- **Fee payer must have funds** — Ensure your FROST group account is funded before signing

---

## Section 5: Signing Session

### 5.1 Coordinator Starts Session

The coordinator initiates the signing session with the transaction file:

```bash
mina-frost-client coordinator \
  -c <CONFIG_PATH> \
  -s <SERVER_URL> \
  -g <GROUP_PUBLIC_KEY> \
  -S <SIGNER_PUBKEY_1>,<SIGNER_PUBKEY_2> \
  -m <TRANSACTION_FILE> \
  -o <SIGNATURE_OUTPUT> \
  -n <NETWORK>
```

**Parameters:**
| Flag | Description |
|------|-------------|
| `-c` | Path to coordinator's config file |
| `-s` | Server URL (optional if stored in group config) |
| `-g` | Group public key (from `groups` command) |
| `-S` | Comma-separated public keys of signers to include |
| `-m` | Path to unsigned transaction JSON file |
| `-o` | Output path for signature (use `-` for stdout) |
| `-n` | Network: `mainnet` or `testnet` (default: `testnet`) |

**Example:**
```bash
mina-frost-client coordinator \
  -c ~/.frost/alice.toml \
  -s localhost:2744 \
  -g <GROUP_PUBLIC_KEY> \
  -S <BOB_PUBKEY>,<EVE_PUBKEY> \
  -m ./tx-json/update-state-transaction.json \
  -o ./signed-tx.json \
  -n testnet
```

### 5.2 Participants Join Session

Each selected signer joins the session:

```bash
mina-frost-client participant \
  -c <CONFIG_PATH> \
  -s <SERVER_URL> \
  -g <GROUP_PUBLIC_KEY> \
  -S <SESSION_ID> \
  -y
```

**Parameters:**
| Flag | Description |
|------|-------------|
| `-c` | Path to participant's config file |
| `-s` | Server URL (optional if stored in group config) |
| `-g` | Group public key |
| `-S` | Session ID (optional if only one active session) |
| `-y` | Auto-approve signing (skip confirmation prompt) |

**Example:**
```bash
# Bob joins
mina-frost-client participant \
  -c ~/.frost/bob.toml \
  -s localhost:2744 \
  -g <GROUP_PUBLIC_KEY> \
  -y

# Eve joins
mina-frost-client participant \
  -c ~/.frost/eve.toml \
  -s localhost:2744 \
  -g <GROUP_PUBLIC_KEY> \
  -y
```

### 5.3 Session Management

**List active sessions:**
```bash
mina-frost-client sessions \
  -c <CONFIG_PATH> \
  -s <SERVER_URL> \
  -g <GROUP_PUBLIC_KEY>
```

**Close all sessions (cleanup):**
```bash
mina-frost-client sessions \
  -c <CONFIG_PATH> \
  -s <SERVER_URL> \
  -g <GROUP_PUBLIC_KEY> \
  --close-all
```

[TODO: Document timing requirements and what happens if participants don't join in time]

---

## Section 6: Broadcasting

### 6.1 Build GraphQL Mutation

Convert the signed transaction to a GraphQL mutation:

```bash
mina-frost-client graphql-build \
  -i <SIGNED_TRANSACTION_JSON> \
  -o <GRAPHQL_OUTPUT_FILE>
```

**Example:**
```bash
mina-frost-client graphql-build \
  -i ./signed-tx.json \
  -o ./broadcast.graphql
```

### 6.2 Broadcast to Network

Submit the GraphQL mutation to a Mina node:

```bash
mina-frost-client graphql-broadcast \
  -g <GRAPHQL_FILE> \
  -e <ENDPOINT_URL>
```

**Example:**
```bash
mina-frost-client graphql-broadcast \
  -g ./broadcast.graphql \
  -e https://api.minascan.io/node/devnet/v1/graphql
```

### GraphQL Endpoints

[TODO: Add list of available GraphQL endpoints for different networks]

| Network | Endpoint |
|---------|----------|
| Mainnet | [TODO] |
| Devnet | `https://api.minascan.io/node/devnet/v1/graphql` |
| Berkeley | [TODO] |

---

## Command Reference

| Command | Description | Key Flags |
|---------|-------------|-----------|
| `init` | Initialize participant config | `-c <config>` |
| `export` | Export contact string | `-n <name>` `-c <config>` |
| `import` | Import a contact | `<contact>` `-c <config>` |
| `contacts` | List contacts | `-c <config>` |
| `remove-contact` | Remove a contact | `-c <config>` `-p <pubkey>` |
| `trusted-dealer` | Test-only key generation | `-c <configs...>` `-d <desc>` `-N <names>` `-t <threshold>` |
| `dkg` | Distributed key generation | `-c <config>` `-d <desc>` `-s <server>` `-t <threshold>` `-S <participants>` |
| `groups` | List groups | `-c <config>` |
| `remove-group` | Remove a group | `-c <config>` `-g <group>` |
| `sessions` | List/manage sessions | `-c <config>` `-s <server>` `-g <group>` `--close-all` |
| `coordinator` | Start signing session | `-c <config>` `-s <server>` `-g <group>` `-S <signers>` `-m <message>` `-o <signature>` `-n <network>` |
| `participant` | Join signing session | `-c <config>` `-s <server>` `-g <group>` `-S <session>` `-y` |
| `graphql-build` | Build GraphQL mutation | `-i <input>` `-o <output>` |
| `graphql-broadcast` | Broadcast transaction | `-g <graphql>` `-e <endpoint>` |

---

## Troubleshooting

### Common Issues

[TODO: Document common error messages and solutions]

| Issue | Possible Cause | Solution |
|-------|---------------|----------|
| Connection refused | Server not running | Start `frostd` server |
| Certificate error | Invalid TLS cert | Regenerate with `mkcert` |
| Group not found | Wrong group key | Run `groups` to list valid keys |
| Session timeout | Participants too slow | [TODO] |

### Debug Tips

[TODO: Add debugging guidance, log locations, verbose mode flags, etc.]

---

## See Also

- [DOC-WORKFLOW.md](./DOC-WORKFLOW.md) — Generating unsigned Mina transactions with o1js
- [README.md](./README.md) — Project overview and repository layout
- [frostd documentation](https://frost.zfnd.org/zcash/server.html) — FROST server setup guide
- [Mina Protocol Documentation](https://docs.minaprotocol.com/) — Official Mina documentation
