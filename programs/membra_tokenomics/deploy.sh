#!/bin/bash
set -e

echo "============================================"
echo "MEMBRA Tokenomics Program Deployment"
echo "============================================"

CLUSTER="${1:-devnet}"
PROGRAM_DIR="$(cd "$(dirname "$0")" && pwd)"

echo ""
echo "[1] Building program..."
cd "$PROGRAM_DIR"
anchor build

echo ""
echo "[2] Deploying to $CLUSTER..."
anchor deploy --provider.cluster "$CLUSTER"

echo ""
echo "[3] Fetching program ID..."
PROGRAM_ID=$(solana address -k "$PROGRAM_DIR/target/deploy/membra_tokenomics-keypair.json")
echo "Program ID: $PROGRAM_ID"

echo ""
echo "[4] Updating declare_id! in lib.rs..."
sed -i.bak "s/declare_id!(\".*\")/declare_id!(\"$PROGRAM_ID\")/" src/lib.rs
rm src/lib.rs.bak

echo ""
echo "[5] Rebuilding with updated program ID..."
anchor build

echo ""
echo "[6] Updating Anchor.toml..."
cd ../../
sed -i.bak "s/membra_tokenomics = \"[^\"]*\"/membra_tokenomics = \"$PROGRAM_ID\"/" Anchor.toml
rm Anchor.toml.bak

echo ""
echo "============================================"
echo "DEPLOYMENT COMPLETE"
echo "============================================"
echo "Program ID: $PROGRAM_ID"
echo "Cluster: $CLUSTER"
echo ""
echo "Next steps:"
echo "  1. Run tests: anchor test --skip-build"
echo "  2. Initialize a sale via the TypeScript SDK"
echo "  3. Update frontend .env with PROGRAM_ID"
