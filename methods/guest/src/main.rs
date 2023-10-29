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
use risc0_zkvm::sha::{Impl, Sha256};
use wordle_core::{Balances, Transaction, AmountLimitPolicy, SenderWhitelistPolicy, Policy};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut _balance_user0: u64 = env::read();
    let mut _balance_user1: u64 = env::read();
    let tx: Transaction = env::read();

    check_policies();
    

    // assert_eq!(
    //     secret.chars().count(),
    //     WORD_LENGTH,
    //     "secret must have length 5!"
    // );

    // assert_eq!(
    //     guess.chars().count(),
    //     WORD_LENGTH,
    //     "guess must have length 5!"
    // );

   // let mut feedback: WordFeedback = WordFeedback::default();

    // to avoid false positive partial matches, create a pool of only letters
    // that didn't have an exact match
    // let mut secret_unmatched: String = String::from("");

    // for i in 0..WORD_LENGTH {
    //     if secret.as_bytes()[i] != guess.as_bytes()[i] {
    //         secret_unmatched.push(secret.as_bytes()[i] as char);
    //    }
    // }

    // // second round for distinguishing partial matches from misses
    // for i in 0..WORD_LENGTH {
    //     feedback.0[i] = if secret.as_bytes()[i] == guess.as_bytes()[i] {
    //         LetterFeedback::Correct
    //     } else if secret_unmatched.as_bytes().contains(&guess.as_bytes()[i]) {
    //         LetterFeedback::Present
    //     } else {
    //         LetterFeedback::Miss
    //     }
    // }

    // let correct_word_hash = *Impl::hash_bytes(&secret.as_bytes());
    // let game_state = GameState {
    //     correct_word_hash,
    //     feedback,
    // };

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
    println!("aa {0} bb {1}", _balance_user0, _balance_user1);

    let balances = Balances {
        balance_user0: _balance_user0,
        balance_user1: _balance_user1
    };
    

    env::commit(&balances);
}

fn check_policies() {
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


