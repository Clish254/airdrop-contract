# Solana Airdrop Program

## Overview
A Solana Anchor program that enables controlled SOL token airdrops from a dedicated vault.

## Features
- Initialize a vault with initial funding
- Airdrop SOL to specified wallets
- Tracks funded and airdropped amounts
- Prevents over-spending with balance checks

## Instructions
- `initialize`: Fund the vault with initial SOL
- `airdrop`: Distribute SOL from the vault to target wallets

## Error Handling
- Prevents transfers exceeding vault balance
- Handles integer overflow scenarios

## Account Structure
- Owner-controlled vault
- PDA vault wallet
- Supports multiple airdrops

## Security
- Uses Anchor's PDA (Program Derived Address) mechanism
- Requires owner signature for vault operations
