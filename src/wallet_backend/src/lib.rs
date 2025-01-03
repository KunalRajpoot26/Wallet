use candid::CandidType;
use ic_cdk::{init, query, storage, update};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Define the structure for an Account with a unique ID and a balance.
#[derive(CandidType, Deserialize, Serialize, Clone, Default)]
struct Account {
    id: String,       // Unique identifier for the account.
    balance: u64,     // Account balance in tokens.
}

// Define the structure for a Ledger to manage accounts and total token supply.
#[derive(CandidType, Deserialize, Serialize, Default)]
struct Ledger {
    accounts: HashMap<String, Account>, // A mapping of account IDs to Account details.
    total_supply: u64,                  // Total supply of tokens in the ledger.
}

// Initialization function for the smart contract.
// This function sets up a default empty ledger and saves it to stable storage.
#[init]
fn init() {
    let ledger = Ledger::default();
    storage::stable_save((ledger,)).unwrap(); // Save the default ledger to stable storage.
}

// Helper function to load the ledger from stable storage.
fn load_ledger() -> Ledger {
    storage::stable_restore::<(Ledger,)>().unwrap_or_default().0
}

// Helper function to save the ledger back to stable storage.
fn save_ledger(ledger: &Ledger) {
    storage::stable_save((ledger,)).unwrap();
}

// Mint new tokens and add them to a specified account.
// If the account does not exist, it is created.
#[update]
fn mint(account_id: String, amount: u64) {
    let mut ledger = load_ledger();
    let account = ledger.accounts.entry(account_id.clone()).or_insert(Account {
        id: account_id,  // Initialize a new account if it doesn't exist.
        balance: 0,
    });
    account.balance += amount;          // Add the minted amount to the account balance.
    ledger.total_supply += amount;      // Increase the total supply by the minted amount.
    save_ledger(&ledger);               // Save the updated ledger to stable storage.
}

// Transfer tokens from one account to another.
// Returns an error if the sender does not have sufficient balance or the account does not exist.
#[update]
fn transfer(from: String, to: String, amount: u64) -> Result<(), String> {
    let mut ledger = load_ledger();

    // Get the sender's account and ensure it exists.
    let from_account = ledger.accounts.get_mut(&from).ok_or("Sender account not found")?;

    // Check if the sender has sufficient balance.
    if from_account.balance < amount {
        return Err("Insufficient balance".to_string());
    }

    // Deduct the amount from the sender's balance.
    from_account.balance -= amount;

    // Add the amount to the recipient's balance.
    let to_account = ledger.accounts.entry(to.clone()).or_insert(Account {
        id: to,          // Initialize a new account for the recipient if it doesn't exist.
        balance: 0,
    });
    to_account.balance += amount;

    save_ledger(&ledger); // Save the updated ledger to stable storage.
    Ok(())
}

// Query the balance of a specific account.
// Returns 0 if the account does not exist.
#[query]
fn balance_of(account_id: String) -> u64 {
    let ledger = load_ledger();
    ledger.accounts.get(&account_id).map_or(0, |account| account.balance)
}

// Query the total supply of tokens in the ledger.
#[query]
fn total_supply() -> u64 {
    let ledger = load_ledger();
    ledger.total_supply
}
