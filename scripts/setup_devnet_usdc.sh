#!/bin/bash

# Finternet SDK - Devnet USDC Setup Script
# This script helps users get USDC on devnet for testing

set -e

echo "🏦 Finternet SDK - Devnet USDC Setup"
echo "===================================="

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Please install it first:"
    echo "   sh -c \"\$(curl -sSfL https://release.solana.com/stable/install)\""
    exit 1
fi

# Check if spl-token is installed
if ! command -v spl-token &> /dev/null; then
    echo "❌ SPL Token CLI not found. Installing..."
    cargo install spl-token-cli
fi

# Get wallet address
WALLET_ADDRESS=$(solana address 2>/dev/null || echo "")
if [ -z "$WALLET_ADDRESS" ]; then
    echo "❌ No wallet found. Creating one..."
    solana-keygen new --no-passphrase --force
    WALLET_ADDRESS=$(solana address)
fi

echo "🔑 Using wallet: $WALLET_ADDRESS"

# Ensure we're on devnet
echo "🌐 Setting Solana CLI to devnet..."
solana config set --url https://api.devnet.solana.com

# Check SOL balance
SOL_BALANCE=$(solana balance --lamports | cut -d' ' -f1)
SOL_AMOUNT=$(echo "scale=4; $SOL_BALANCE / 1000000000" | bc -l 2>/dev/null || echo "0")

echo "💰 SOL Balance: $SOL_AMOUNT SOL"

if (( $(echo "$SOL_AMOUNT < 0.1" | bc -l) )); then
    echo "🪂 Getting SOL airdrop..."
    solana airdrop 2
    sleep 3
fi

# USDC mint address on devnet
USDC_MINT="4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"

echo "💵 Setting up USDC token account..."

# Create USDC token account if it doesn't exist
USDC_ACCOUNT=$(spl-token accounts --token $USDC_MINT --output json 2>/dev/null | jq -r '.accounts[0].address // empty' || echo "")

if [ -z "$USDC_ACCOUNT" ]; then
    echo "📝 Creating USDC token account..."
    USDC_ACCOUNT=$(spl-token create-account $USDC_MINT --output json | jq -r '.account')
    echo "✅ Created USDC account: $USDC_ACCOUNT"
else
    echo "✅ Found existing USDC account: $USDC_ACCOUNT"
fi

# Check USDC balance
USDC_BALANCE=$(spl-token balance $USDC_MINT 2>/dev/null || echo "0")
echo "💰 Current USDC Balance: $USDC_BALANCE USDC"

if (( $(echo "$USDC_BALANCE < 10" | bc -l) )); then
    echo ""
    echo "🎯 Getting USDC from faucets:"
    echo "============================================="
    echo "1. SPL Token Faucet:"
    echo "   https://spl-token-faucet.com/?token-name=USDC"
    echo ""
    echo "2. Solana Discord #devnet-faucet:"
    echo "   !faucet $WALLET_ADDRESS USDC 100"
    echo ""
    echo "3. Manual mint (if you have authority):"
    echo "   spl-token mint $USDC_MINT 100 $USDC_ACCOUNT"
    echo ""
    echo "🔧 After getting USDC, test with:"
    echo "   cargo run --bin finternet-cli setup-usdc"
    echo "   cargo run --example enhanced_demo"
else
    echo "🎉 You have sufficient USDC for testing!"
    echo ""
    echo "🚀 Ready to run demos:"
    echo "   cargo run --example enhanced_demo"
    echo "   cargo run --bin finternet-cli demo"
fi

echo ""
echo "📋 Summary:"
echo "   Wallet: $WALLET_ADDRESS"
echo "   SOL: $SOL_AMOUNT"
echo "   USDC: $USDC_BALANCE"
echo "   USDC Account: $USDC_ACCOUNT" 