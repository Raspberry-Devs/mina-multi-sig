# Mina zkApp Transaction Generator - Workflow Documentation

## Project Overview
This project generates unsigned Mina zkApp transactions as JSON files for external signing and deployment.

---

## Project Structure

```
mina-tx-generator/
├── src/
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
      let senderUpdate = AccountUpdate.fundNewAccount(sender);
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
  Bool,
} from 'o1js';
import * as fs from 'fs';

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
    const currentValue = Field(0);
    this.counter.requireEquals(currentValue);
    this.counter.set(currentValue.add(1));
  }
}

async function generateUpdateStateTx() {
  // 1. Setup LocalBlockchain (proofsEnabled: false for signature-based)
  const Local = await Mina.LocalBlockchain({ proofsEnabled: false });
  Mina.setActiveInstance(Local);

  // 2. Get accounts
  const [deployer] = Local.testAccounts;
  const contractAccount = Mina.TestPublicKey.random();
  const contract = new StateContract(contractAccount);

  // 3. Compile (needed for verification key)
  await StateContract.compile();

  // 4. Create deploy transaction (unsigned)
  const deployTx = await Mina.transaction(deployer, async () => {
    AccountUpdate.fundNewAccount(deployer);
    await contract.deploy();
  });
  fs.writeFileSync('./tx-json/deploy-state-contract.json', deployTx.toJSON());
  console.log('Deploy transaction saved to ./tx-json/deploy-state-contract.json');

  // 5. Create update transaction (unsigned)
  const tx = await Mina.transaction(deployer, async () => {
    await contract.incrementCounter();
  });

  // Set useFullCommitment to true for FROST signing
  const contractAccountUpdate = tx.transaction.accountUpdates[0];
  contractAccountUpdate.body.useFullCommitment = Bool(true);

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
  Bool,
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

async function generateUpdatePermissionsTx() {
  // 1. Setup LocalBlockchain (proofsEnabled: false for signature-based)
  const Local = await Mina.LocalBlockchain({ proofsEnabled: false });
  Mina.setActiveInstance(Local);

  // 2. Get accounts
  const [deployer] = Local.testAccounts;
  const contractAccount = Mina.TestPublicKey.random();
  const contract = new PermissionsContract(contractAccount);

  // 3. Compile (needed for verification key)
  await PermissionsContract.compile();

  // 4. Create deploy transaction (unsigned)
  const deployTx = await Mina.transaction(deployer, async () => {
    AccountUpdate.fundNewAccount(deployer);
    await contract.deploy();
  });
  fs.writeFileSync('./tx-json/deploy-permissions-contract.json', deployTx.toJSON());
  console.log('Deploy transaction saved to ./tx-json/deploy-permissions-contract.json');

  // 5. Create update transaction (unsigned)
  const tx = await Mina.transaction(deployer, async () => {
    await contract.updatePermissions();
  });

  // Set useFullCommitment to true for FROST signing
  const contractAccountUpdate = tx.transaction.accountUpdates[0];
  contractAccountUpdate.body.useFullCommitment = Bool(true);

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
  Bool,
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

class NewContract extends SmartContract {
  @method async dummy() {
    // Different contract = different verification key
  }
}

async function generateUpdateVerificationKeyTx() {
  // 1. Setup LocalBlockchain (proofsEnabled: false for signature-based)
  const Local = await Mina.LocalBlockchain({ proofsEnabled: false });
  Mina.setActiveInstance(Local);

  // 2. Get accounts
  const [deployer] = Local.testAccounts;
  const contractAccount = Mina.TestPublicKey.random();
  const contract = new UpdatableContract(contractAccount);

  // 3. Compile original contract (needed for verification key)
  await UpdatableContract.compile();

  // 4. Create deploy transaction (unsigned)
  const deployTx = await Mina.transaction(deployer, async () => {
    AccountUpdate.fundNewAccount(deployer);
    await contract.deploy();
  });
  fs.writeFileSync('./tx-json/deploy-updatable-contract.json', deployTx.toJSON());
  console.log('Deploy transaction saved to ./tx-json/deploy-updatable-contract.json');

  // 5. Compile new contract for different verification key
  const { verificationKey: newVerificationKey } = await NewContract.compile();

  // 6. Create update transaction (unsigned)
  const tx = await Mina.transaction(deployer, async () => {
    await contract.updateVerificationKey(newVerificationKey);
  });

  // Set useFullCommitment to true for FROST signing
  const contractAccountUpdate = tx.transaction.accountUpdates[0];
  contractAccountUpdate.body.useFullCommitment = Bool(true);

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
- **`Mina.TestPublicKey.random()`** — generates a random keypair for the contract account

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
