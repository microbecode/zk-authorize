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

use std::io;

use risc0_zkvm::{default_prover, serde::from_slice, ExecutorEnv, Receipt};
use zkauth_core::{Balances, Transaction};
use zkauth_methods::{ZKAUTH_GUEST_ELF, ZKAUTH_GUEST_ID};

struct Server {
    balance_user0: u64,
    balance_user1: u64
}

impl Server {
    pub fn new() -> Self {
        Self { 
            balance_user0: 10_u64, 
            balance_user1: 10_u64 
        }
    }

    pub fn transfer(&self, source: u64, target: u64, amount: u64) -> Receipt {
        let tx = Transaction {
            from: source,
            to: target,
            amount: amount
        };

        let env = ExecutorEnv::builder()
            .write(&self.balance_user0)
            .unwrap()
            .write(&self.balance_user1)
            .unwrap()
            .write(&tx)
            .unwrap()
            .build()
            .unwrap();

        // Obtain the default prover.
        let prover = default_prover();

        // Produce a receipt by proving the specified ELF binary.
        prover.prove_elf(env, ZKAUTH_GUEST_ELF).unwrap()
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::{Server, Balances};

    #[test]
    #[serial]
    fn main() {
        let server = Server::new();
        server.transfer(0, 1, 4);
    }
}
