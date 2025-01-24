# Anchor Vault Program ⚓️

## Overview

The Anchor Vault Program is a decentralized application built on the Solana blockchain using the Anchor framework. This program allows users to create vaults where they can deposit and withdraw funds securely. It provides functionalities for initializing a vault, depositing a specified amount, withdrawing a specific amount, and withdrawing all funds at once.

## Structure of Program

<img src="images/project_structure.png" alt="Project Structure" width="900" height="500"/>
<br/>

## Anchor Program TestCases ✅

<img src="https://github.com/baindlapranayraj/AnchorVault/blob/main/test-cases.png" alt="Project Structure" width="900" height="500"/>
<br/>

## Modules

This program consists of several modules:

- **constants**: Contains constant values used throughout the program.
- **error**: Defines custom error types for better error handling.
- **instructions**: Implements the logic for various instructions (initialize, deposit, withdraw).
- **state**: Defines the state structure of the vault.

## All Instructions 

### 1. Initialize Vault

- Initializes a new vault account.
- Requires the context containing the necessary accounts.

### 2. Deposit Funds

- Allows users to deposit a specified amount into the vault.
- `amount`: The amount of funds to deposit (in lamports).

### 3. Withdraw Specific Amount

- Allows users to withdraw a specific amount from their vault.
- `amount`: The amount of funds to withdraw (in lamports).

### 4. Withdraw All Funds

- Allows users to withdraw a all of his amount from their vault.


