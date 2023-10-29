// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use risc0_zkvm::guest::env;
use zkauth_core::{Balances, Transaction, AmountLimitPolicy, SenderWhitelistPolicy, Policy};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut _balance_user0: u64 = env::read();
    let mut _balance_user1: u64 = env::read();
    let tx: Transaction = env::read();

    check_policies(&tx);
    
    if tx.from == 0 {
        if tx.to == 1 {
            _balance_user0 -= tx.amount;
            _balance_user1 += tx.amount;
        }          
    }
    else {
        if tx.to == 0 {
            _balance_user1 -= tx.amount;
            _balance_user0 += tx.amount;
        }
    }

    let balances = Balances {
        balance_user0: _balance_user0,
        balance_user1: _balance_user1
    };    

    env::commit(&balances);
}

fn check_policies(tx : &Transaction) {
    let mut policies: Vec<Box<dyn Policy>> = Vec::new();

    policies.push(Box::new(AmountLimitPolicy { max_amount: 1000 }));
    let mut whitelisted : Vec<u64> = Vec::new();
    whitelisted.push(0);
    whitelisted.push(1);
    policies.push(Box::new(SenderWhitelistPolicy { whitelisted_senders: whitelisted }));

    assert!(validate_transaction(&tx, &policies), "Invalid transaction");
}

fn validate_transaction(transaction: &Transaction, policies: &Vec<Box<dyn Policy>>) -> bool {
    for policy in policies {
        if !policy.is_valid(transaction.clone()) {
            return false;
        }
    }
    true
}


