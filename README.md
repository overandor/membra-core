# MEMBRA Core Engine

Military-grade C++ asset management system for MEMBRA protocol.

## Overview

MEMBRA Core is a buildable, testable C++ engine that implements the core MEMBRA functionality:

- **Asset ingestion** with SHA-256 hashing
- **Custody logging** for chain of custody tracking
- **Change tracking** for audit trails
- **Word-to-function economic appraisal** with evidence requirements
- **Proof bundle generation** with Merkle trees
- **Policy enforcement** for military-grade compliance
- **Terminal ticker** for asset metrics display
- **LLM routing hooks** (disabled by default - no external network calls)
- **Incident management** for security events

## Hard Rules

- No fake success output
- No mock production paths
- No simulated settlement counted as real
- No fake liquidity
- No fake revenue
- No private keys
- No external network calls by default
- No destructive file operations
- No appraising un-hashed files
- No proof bundle without custody record
- No value score without evidence status

## Build Requirements

- CMake 3.15+
- C++17 compiler (Clang on macOS)
- OpenSSL (for SHA-256)
- nlohmann/json (auto-fetched by CMake)

## Building

```bash
cd /Users/alep/Downloads/membra-core
mkdir -p build
cd build
cmake ..
cmake --build .
```

## Usage

### Initialize Workspace

```bash
./membra-core init ./test-workspace
```

### Ingest File

```bash
./membra-core ingest ../tests/fixtures/sample.txt
```

### Appraise Word

```bash
./membra-core appraise-word liquidity
```

This will fail with:
```
Error: Word 'liquidity' requires settlement evidence.
Required evidence: settlement_proof, revenue_data
Refusing to claim liquidity without settlement evidence.
```

### Appraise Asset

```bash
./membra-core appraise-asset <asset_id>
```

### Create Proof Bundle

```bash
./membra-core prove <asset_id>
```

### Terminal Overview

```bash
./membra-core terminal-overview
```

## Acceptance Test

```bash
mkdir -p build
cd build
cmake ..
cmake --build .
./membra-core init ./test-workspace
./membra-core ingest ../tests/fixtures/sample.txt
./membra-core appraise-word liquidity
./membra-core terminal-overview
```

The system passes only if:

1. It compiles
2. It creates the workspace
3. It hashes a real file
4. It writes a custody event
5. It appraises the word liquidity with evidence requirements
6. It refuses to claim liquidity without settlement evidence
7. It creates a proof bundle only after hash and custody exist
8. It outputs terminal-style asset metrics

## Integration Tests

```bash
# Build with tests
cd build
cmake .. -DBUILD_TESTS=ON
cmake --build .

# Run integration tests
./integration_test
```

## Project Structure

```
membra-core/
├── CMakeLists.txt
├── README.md
├── include/membra/
│   ├── models.hpp
│   ├── asset_hasher.hpp
│   ├── asset_store.hpp
│   ├── custody_log.hpp
│   ├── change_tracker.hpp
│   ├── word_registry.hpp
│   ├── word_function_mapper.hpp
│   ├── word_appraiser.hpp
│   ├── economic_capacity_scorer.hpp
│   ├── policy_engine.hpp
│   ├── proof_engine.hpp
│   ├── merkle_log.hpp
│   ├── terminal_ticker_engine.hpp
│   ├── llm_router.hpp
│   └── incident_manager.hpp
├── src/
│   ├── main.cpp
│   ├── asset_hasher.cpp
│   ├── asset_store.cpp
│   ├── custody_log.cpp
│   ├── change_tracker.cpp
│   ├── word_registry.cpp
│   ├── word_function_mapper.cpp
│   ├── word_appraiser.cpp
│   ├── economic_capacity_scorer.cpp
│   ├── policy_engine.cpp
│   ├── proof_engine.cpp
│   ├── merkle_log.cpp
│   ├── terminal_ticker_engine.cpp
│   ├── llm_router.cpp
│   └── incident_manager.cpp
├── tests/
│   ├── integration_test.cpp
│   └── fixtures/
│       └── sample.txt
└── examples/
```

## Word Economic Formula

```
word_economic_capacity =
    semantic_power * 0.15
    + actionability * 0.20
    + proof_linkage * 0.20
    + buyer_relevance * 0.15
    + automation_value * 0.15
    + settlement_relevance * 0.10
    + risk_control_value * 0.05
```

Hard zero if:
- The word creates a false claim
- The word cannot attach to evidence
- The word has no function
- The word cannot contribute to work, proof, revenue, risk reduction, settlement, or service delivery

## Word-to-Function Mapping

| Word | Function |
|------|----------|
| file | register_asset |
| asset | create_asset_record |
| hash | generate_hash |
| proof | create_proof_bundle |
| work | create_work_unit |
| llm | record_model_work |
| settlement | record_settlement |
| liquidity | calculate_liquidity_score |
| cicd | optimize_pipeline |
| security | score_security_improvement |
| chain | prepare_audit_anchor |
| revenue | attribute_revenue |
| custody | record_custody_event |
| change | record_change_event |

## Data Flow

```
file_path
  → AssetHasher
  → AssetStore
  → CustodyLog
  → PolicyEngine
  → WordRegistry / AppraisalEngine
  → ProofEngine
  → TerminalTickerEngine
```

## Proof Bundle Files

- asset_manifest.json
- custody_chain.jsonl
- change_history.jsonl
- word_appraisal.json
- policy_decision.json
- hashes.json
- merkle_root.txt
- README.md

## License

MIT
