# Mina zkApp Transaction Generator - Workflow Documentation

## Project Overview
This project generates unsigned Mina zkApp transactions as JSON files for external signing and deployment.

---

## Project Structure

```
mina-tx-generator/
├── src/
│   ├── commit.ts                  # Update transaction fields
│   ├── payment.ts                 # Simple payment transaction
│   ├── update_state.ts            # zkApp state modification
│   ├── update_permissions.ts      # zkApp permissions update
│   └── update_verification_key.ts # zkApp verification key update
├── tx-json/                       # Output directory for unsigned transactions
├── package.json
└── tsconfig.json
```

---

## Dependencies

```json
{
  "dependencies": {
    "o1js": "^1.4.1"
  },
  "devDependencies": {
    "@types/node": "^20.4.2",
    "ts-node": "^10.9.2",
    "typescript": "^5.2.2"
  }
}
```

**tsconfig.json requirements:**
```json
{
  "compilerOptions": {
    "target": "es2021",
    "module": "nodenext",
    "strict": true,
    "esModuleInterop": true,
    "experimentalDecorators": true,
    "emitDecoratorMetadata": true
  }
}
```

---

## NPM Scripts

```bash
npm run payment           # Generate payment transaction
npm run update-state      # Generate state update transactions
npm run update-permissions # Generate permissions update transactions
npm run update-vk         # Generate verification key update transactions
```

---

## Output Files

| Script | Output Files |
|--------|--------------|
| `update-state` | `tx-json/deploy-state-contract.json`, `tx-json/update-state-transaction.json` |
| `update-permissions` | `tx-json/deploy-permissions-contract.json`, `tx-json/update-permissions-transaction.json` |
| `update-vk` | `tx-json/deploy-updatable-contract.json`, `tx-json/update-verification-key-transaction.json` |
| `payment` | `tx-json/payment-transaction.json` |

---

## Transaction JSON Structure

Each JSON file contains:
- `feePayer`: object with `publicKey` field (signature is empty/null)
- `accountUpdates`: array of account updates for the transaction
- `memo`: transaction memo field

---

## Signing Requirements

| Transaction Type | Required Signatures |
|-----------------|---------------------|
| Deploy transactions | Fee payer key + Contract account key |
| Interaction transactions (state/permissions/vk update) | Fee payer key + Contract account key (due to `requireSignature()`) |
| Payment transactions | Fee payer key |

---

## Source Code
### src/commit.ts
```typescript
import { Bool, Mina } from "o1js";

export function UpdateFullCommitment(...tx: Mina.Transaction<false, false>[]) {
  tx.forEach((transaction) => {
    transaction.transaction.accountUpdates.map(
      (au) => au.body.useFullCommitment = Bool(true)
    );
  });
}
```

### src/payment.ts
```typescript
import { AccountUpdate, Mina, PrivateKey, PublicKey, UInt64 } from 'o1js';
import * as fs from 'fs';

async function createPayment() {

  let Local = await Mina.LocalBlockchain({ proofsEnabled: true });
  Mina.setActiveInstance(Local);

  let [sender, receiver] = Local.testAccounts;


  const tx = await Mina.transaction(
    sender,
    async () => {
      senderUpdate.send({ to: receiver, amount: UInt64.from(1e9) });
    }
  );

  await tx.prove(); // optional for basic payment
  const json = tx.toJSON();

  const outputPath = './tx-json/payment-transaction.json';
  fs.writeFileSync(outputPath, json);
  console.log(`Transaction saved to ${outputPath}`);
}

createPayment();
```

### src/update_state.ts
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
import { UpdateFullCommitment } from './commit';

const MINA_TESTNET_URL = 'https://api.minascan.io/node/devnet/v1/graphql';
const FEE = 100_000_000; // 0.1 MINA
const DEPLOYER_KEY = '<PUBLIC_KEY>';

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
    this.requireSignature();
    const currentValue = this.counter.get();
    this.counter.requireEquals(currentValue);
    this.counter.set(currentValue.add(1));
  }
}

async function generateUpdateStateTx() {
  // 1. Setup Mina testnet
  const network = Mina.Network(MINA_TESTNET_URL);
  Mina.setActiveInstance(network);

  // 2. Get accounts
  const deployer = PublicKey.fromBase58(DEPLOYER_KEY);
  const contractAccount = PublicKey.fromBase58(DEPLOYER_KEY);
  const contract = new StateContract(contractAccount);

  // 3. Compile (needed for verification key)
  await StateContract.compile();

  // 4. Create deploy transaction (unsigned)
  const deployTx = await Mina.transaction(
    { sender: deployer, fee: FEE },
    async () => {
      await contract.deploy();
    }
  );

  // 5. Create update transaction (unsigned)
  const tx = await Mina.transaction(
    { sender: deployer, fee: FEE },
    async () => {
      await contract.incrementCounter();
    }
  );

  UpdateFullCommitment(deployTx, tx);

  fs.writeFileSync('./tx-json/deploy-state-contract.json', deployTx.toJSON());
  console.log('Deploy transaction saved to ./tx-json/deploy-state-contract.json');

  fs.writeFileSync('./tx-json/update-state-transaction.json', tx.toJSON());
  console.log('Update transaction saved to ./tx-json/update-state-transaction.json');
}

generateUpdateStateTx();
```

### src/update_permissions.ts
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
import { UpdateFullCommitment } from './commit';

const MINA_TESTNET_URL = 'https://api.minascan.io/node/devnet/v1/graphql';
const FEE = 100_000_000; // 0.1 MINA
const DEPLOYER_KEY = '<PUBLIC_KEY>';

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

async function generateUpdatePermissionsTx() {
  // 1. Setup Mina testnet
  const network = Mina.Network(MINA_TESTNET_URL);
  Mina.setActiveInstance(network);

  // 2. Get accounts
  const deployer = PublicKey.fromBase58(DEPLOYER_KEY);
  const contractAccount = PublicKey.fromBase58(DEPLOYER_KEY);
  const contract = new PermissionsContract(contractAccount);

  // 3. Compile (needed for verification key)
  await PermissionsContract.compile();

  // 4. Create deploy transaction (unsigned)
  const deployTx = await Mina.transaction(
    { sender: deployer, fee: FEE },
    async () => {
      await contract.deploy();
    }
  );

  // 5. Create update transaction (unsigned)
  const tx = await Mina.transaction(
    { sender: deployer, fee: FEE },
    async () => {
      await contract.updatePermissions();
    }
  );

  UpdateFullCommitment(deployTx, tx);

  fs.writeFileSync('./tx-json/deploy-permissions-contract.json', deployTx.toJSON());
  console.log('Deploy transaction saved to ./tx-json/deploy-permissions-contract.json');

  fs.writeFileSync('./tx-json/update-permissions-transaction.json', tx.toJSON());
  console.log('Update transaction saved to ./tx-json/update-permissions-transaction.json');
}

generateUpdatePermissionsTx();
```

### src/update_verification_key.ts
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
import { UpdateFullCommitment } from './commit';

const MINA_TESTNET_URL = 'https://api.minascan.io/node/devnet/v1/graphql';
const FEE = 100_000_000; // 0.1 MINA
const DEPLOYER_KEY = '<PUBLIC_KEY>';

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

class NewContract extends SmartContract {
  @method async dummy() {
    // Different contract = different verification key
  }
}

async function generateUpdateVerificationKeyTx() {
  // 1. Setup Mina testnet
  const network = Mina.Network(MINA_TESTNET_URL);
  Mina.setActiveInstance(network);

  // 2. Get accounts
  const deployer = PublicKey.fromBase58(DEPLOYER_KEY);
  const contractAccount = PublicKey.fromBase58(DEPLOYER_KEY);
  const contract = new UpdatableContract(contractAccount);

  // 3. Compile original contract (needed for verification key)
  await UpdatableContract.compile();

  // 4. Create deploy transaction (unsigned)
  const deployTx = await Mina.transaction(
    { sender: deployer, fee: FEE },
    async () => {
      await contract.deploy();
    }
  );

  // 5. Compile new contract for different verification key
  const { verificationKey: newVerificationKey } = await NewContract.compile();

  // 6. Create update transaction (unsigned)
  const tx = await Mina.transaction(
    { sender: deployer, fee: FEE },
    async () => {
      await contract.updateVerificationKey(newVerificationKey);
    }
  );

  UpdateFullCommitment(deployTx, tx);

  fs.writeFileSync('./tx-json/deploy-updatable-contract.json', deployTx.toJSON());
  console.log('Deploy transaction saved to ./tx-json/deploy-updatable-contract.json');

  fs.writeFileSync('./tx-json/update-verification-key-transaction.json', tx.toJSON());
  console.log('Update transaction saved to ./tx-json/update-verification-key-transaction.json');
}

generateUpdateVerificationKeyTx();
```

---

## Key Technical Details

- **`proofsEnabled: false`** — transactions use signatures instead of zero-knowledge proofs
- **`Permissions.signature()`** — allows authorization via signature instead of proof
- **`this.requireSignature()`** — method call that requires contract account signature for authorization
- **Compilation is still required** — generates verification key needed for contract deployment

---

## External Signing Workflow

1. **Generate transactions**: Run npm script → outputs unsigned JSON to `./tx-json/`
2. **Sign externally**: External tool reads JSON, signs with required private keys
3. **Submit**: Signed transaction submitted to Mina network

---

## o1js APIs Used

| API | Purpose |
|-----|---------|
| `SmartContract` | Base class for zkApps |
| `@method` decorator | Defines callable contract methods |
| `@state(Field)` decorator | Declares on-chain state variables |
| `State<T>` | Type wrapper for state variables |
| `VerificationKey` | Type for verification keys |
| `Permissions` | Permission configuration object |
| `Permissions.signature()` | Require signature for authorization |
| `Permissions.VerificationKey.signature()` | Require signature for VK updates |
| `this.account.permissions.set()` | Update account permissions |
| `this.account.verificationKey.set()` | Update verification key |
| `this.requireSignature()` | Require contract key signature in method |
| `Mina.LocalBlockchain()` | Create local test blockchain |
| `Mina.transaction()` | Create a transaction |
| `AccountUpdate.fundNewAccount()` | Fund a new account (pays creation fee) |
| `tx.toJSON()` | Serialize transaction to JSON |
