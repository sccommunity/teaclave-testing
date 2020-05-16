// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![cfg_attr(feature = "sgx", no_std)]
#[cfg(feature = "sgx")]
#[macro_use]
extern crate sgx_tstd as std;

use std::string::String;
use std::vec::Vec;

pub use testing_proc_macro::test;

pub struct TestCase(pub String, pub fn() -> ());

pub use inventory::*;

inventory::collect!(TestCase);

pub fn run() -> bool {
    run_partially(|_| true)
}

pub fn run_partially<F>(match_string: F) -> bool
where
    F: Fn(&str) -> bool,
{
    use std::prelude::v1::*;

    crate::test_start();
    let mut ntestcases: u64 = 0u64;
    let mut failurecases: Vec<String> = Vec::new();

    for t in inventory::iter::<TestCase>.into_iter() {
        if !match_string(&t.0) {
            continue;
        }

        crate::test(&mut ntestcases, &mut failurecases, t.1, &t.0);
    }

    crate::test_end(ntestcases, failurecases)
}

#[macro_export]
macro_rules! generate_runner {
    ($matcher:expr) => {
        pub fn run() -> bool {
            $crate::run_partially($matcher)
        }
    };
    () => {
        generate_runner!(|_| true);
    };
}

#[macro_export]
macro_rules! run_tests {
    ($predicate:expr) => {{
        use std::prelude::v1::*;

        $crate::test_start();
        let mut ntestcases: u64 = 0u64;
        let mut failurecases: Vec<String> = Vec::new();

        for t in $crate::iter::<$crate::TestCase>.into_iter() {
            if $predicate(&t.0) {
                $crate::test(&mut ntestcases, &mut failurecases, t.1, &t.0);
            }
        }

        $crate::test_end(ntestcases, failurecases)
    }};
    () => {
        run_tests!(|_| true);
    };
}

#[macro_export]
macro_rules! should_panic {
    ($fmt:expr) => {{
        match ::std::panic::catch_unwind(|| $fmt).is_err() {
            true => {
                println!(
                    "{} {} ... {}!",
                    "testing_should_panic",
                    stringify!($fmt),
                    "\x1B[1;32mok\x1B[0m"
                );
            }
            false => ::std::rt::begin_panic($fmt),
        }
    }};
}

#[macro_export]
macro_rules! check_all_passed {
    (
        $($f : expr),* $(,)?
    ) => {
        {
            let mut v: Vec<bool> = Vec::new();
            $(
                v.push($f);
            )*
            v.iter().all(|&x| x)
        }
    }
}

/*
#[macro_export]
macro_rules! run_tests {
    (
        $($f : expr),* $(,)?
    ) => {
        {
            teaclave_test_utils::test_start();
            let mut ntestcases : u64 = 0u64;
            let mut failurecases : Vec<String> = Vec::new();
            $(teaclave_test_utils::test(&mut ntestcases, &mut failurecases, $f,stringify!($f));)*
            teaclave_test_utils::test_end(ntestcases, failurecases)
        }
    }
}
*/

pub fn test_start() {
    println!("\nstart running tests");
}

pub fn test_end(ntestcases: u64, failurecases: Vec<String>) -> bool {
    let ntotal = ntestcases as usize;
    let nsucc = ntestcases as usize - failurecases.len();

    if !failurecases.is_empty() {
        print!("\nfailures: ");
        println!(
            "    {}",
            failurecases
                .iter()
                .fold(String::new(), |s, per| s + "\n    " + per)
        );
    }

    if ntotal == nsucc {
        print!("\ntest result \x1B[1;32mok\x1B[0m. ");
    } else {
        print!("\ntest result \x1B[1;31mFAILED\x1B[0m. ");
    }

    println!(
        "{} tested, {} passed, {} failed",
        ntotal,
        nsucc,
        ntotal - nsucc
    );
    failurecases.is_empty()
}

#[allow(clippy::print_literal)]
pub fn test<F, R>(ncases: &mut u64, failurecases: &mut Vec<String>, f: F, name: &str)
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
{
    *ncases += 1;
    let t = || {
        f();
    };
    if std::panic::catch_unwind(t).is_ok() {
        println!("{} {} ... {}!", "testing", name, "\x1B[1;32mok\x1B[0m");
    } else {
        println!("{} {} ... {}!", "testing", name, "\x1B[1;31mfailed\x1B[0m");
        failurecases.push(String::from(name));
    }
}
