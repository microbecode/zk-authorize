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

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Balances {
    pub balance_user0: u64,
    pub balance_user1: u64
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Transaction {
    pub from: u64,
    pub to: u64,
    pub amount: u64
}

pub trait Policy {
    fn is_valid(&self, tx: Transaction) -> bool;
}

pub struct AmountLimitPolicy {
    pub max_amount: u64,
}

pub struct SenderWhitelistPolicy {
    pub whitelisted_senders: Vec<u64>,
}

impl Policy for AmountLimitPolicy {
    fn is_valid(&self, transaction: Transaction) -> bool {
        transaction.amount <= self.max_amount
    }
}

impl Policy for SenderWhitelistPolicy {
    fn is_valid(&self, transaction: Transaction) -> bool {
        self.whitelisted_senders.contains(&transaction.from)
    }
}