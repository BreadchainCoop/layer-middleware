# Validate Signature Utility

A Rust utility for validating signatures from multiple operators in the Hello World AVS (Actively Validated Service).

## Overview

This utility provides functionality to:
1. Read private keys from operator files
2. Generate sorted operator signatures
3. Submit signature validation to the blockchain

## Sequence Diagram

```mermaid
sequenceDiagram
    participant Client
    participant KeyReader
    participant SignatureValidator
    participant Blockchain

    Client->>SignatureValidator: validate_signature("message")
    SignatureValidator->>KeyReader: read_private_keys()
    loop For each key file
        KeyReader->>KeyReader: Read ~/.nodes/operatorN
        KeyReader->>KeyReader: Parse PRIVATE_KEY
        KeyReader->>KeyReader: Format & validate key
    end
    KeyReader-->>SignatureValidator: Return keys vector
    
    SignatureValidator->>SignatureValidator: Sort operator addresses
    loop For each sorted operator
        SignatureValidator->>SignatureValidator: Generate signature
    end
    
    SignatureValidator->>Blockchain: validateOffchainMessage(
        dataHash,
        signatureData
    )
    Blockchain-->>SignatureValidator: Transaction hash
    SignatureValidator-->>Client: Result
```

## Usage

### Running the Utility

```bash
cargo run --bin validate_signature
```

### Environment Setup

1. Use expected operator key files in `~/.nodes/` directory:
   ```
   ~/.nodes/operator1
   ~/.nodes/operator2
   ~/.nodes/operator3
   ```

2. Each operator file should contain a private key in the format:
   ```
   PRIVATE_KEY_1=0x...
   ```

### Configuration

- RPC URL: Defaults to `http://ethereum:8545`
- Can be overridden with `TESTNET_RPC_URL` environment variable

## Key Features

1. **Sorted Operator Processing**
   - Operators are sorted by address before signature generation
   - Ensures consistent ordering for contract validation

2. **Multi-Signature Support**
   - Handles multiple operator signatures
   - Creates combined signature data for contract verification

3. **Error Handling**
   - Validates key formats
   - Ensures proper key lengths (64 characters without '0x' prefix)
   - Provides detailed error messages

## Function Flow

1. `read_private_keys()`
   - Reads keys from operator files
   - Validates and formats keys
   - Returns vector of formatted keys

2. `validate_signature()`
   - Creates sorted operator list
   - Generates signatures in order
   - Submits combined data to blockchain

## Contract Integration

Integrates with:
- OffchainMessageConsumer