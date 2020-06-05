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

pub struct TestCase {
    pub id: String,
    pub func: fn() -> (),
    pub should_panic: Option<String>,
    pub ignored: bool,
}

pub use inventory::*;

inventory::collect!(TestCase);

impl TestCase {
    pub fn new(id: &str, func: fn() -> (), should_panic: Option<&str>, ignored: bool) -> Self {
        use std::string::ToString;

        Self {
            id: id.to_string(),
            func,
            should_panic: should_panic.map(|s| s.to_string()),
            ignored,
        }
    }
}

pub fn run() -> bool {
    run_partially(|_| true)
}

pub fn run_partially<F>(match_string: F) -> bool
where
    F: Fn(&str) -> bool,
{
    use std::prelude::v1::*;

    crate::start(crate::iter::<TestCase>.into_iter().count());

    let mut npassed = 0usize;
    let mut nignored = 0usize;
    let mut failurecases: Vec<String> = Vec::new();

    for c in crate::iter::<TestCase>.into_iter() {
        if !match_string(&c.id) {
            continue;
        } else if c.ignored {
            nignored += 1;
            println!("test {} ... \x1B[1;33mignore\x1B[0m", c.id);
            continue;
        }

        if crate::test(&c) {
            npassed += 1;
        } else {
            failurecases.push(c.id.clone());
        }
    }

    crate::end(npassed, nignored, failurecases)
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

// this macro seems unneccessary as temporary fix proposed at
// https://github.com/apache/incubator-teaclave-sgx-sdk/issues/232#issuecomment-623804958
/*
#[macro_export]
macro_rules! generate_runner_main {
    (
        //$($modules:path),* $(,)?
        $modules:path
    ) => {
        pub fn run() -> bool {
            // rename due to restriction that path cannot be followed by `::`
            // @see https://doc.rust-lang.org/1.7.0/book/macros.html#syntactic-requirements
            //{
            //    use $modules as m;
            //}
            //m::run();

            $crate::run_partially(|_| true);

            true
        }
    };
}
*/

/*
#[macro_export]
macro_rules! run_tests {
    ($predicate:expr) => {{
        use std::prelude::v1::*;

        $crate::start($crate::iter::<$crate::TestCase>.into_iter().count());

        let mut ntestcases: u64 = 0u64;
        let mut failurecases: Vec<String> = Vec::new();

        for t in $crate::iter::<$crate::TestCase>.into_iter() {
            if $predicate(&t.id) {
                $crate::test(&mut ntestcases, &mut failurecases, &t);
            }
        }

        $crate::test_end(ntestcases, failurecases)
    }};
    () => {
        run_tests!(|_| true);
    };
}
*/

//#[macro_export]
//macro_rules! should_panic {
//    ($fmt:expr) => {{
//        match ::std::panic::catch_unwind(|| $fmt).is_err() {
//            true => {
//                println!(
//                    "{} {} ... {}!",
//                    "testing_should_panic",
//                    stringify!($fmt),
//                    "\x1B[1;32mok\x1B[0m"
//                );
//            }
//            false => ::std::rt::begin_panic($fmt),
//        }
//    }};
//}

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

pub fn start(n: usize) {
    println!("\nrunning {} tests", n);
}

pub fn end(npassed: usize, nignored: usize, failurecases: Vec<String>) -> bool {
    //let ntotal = ntestcases as usize;
    //let nsucc = ntestcases as usize - failurecases.len();

    if !failurecases.is_empty() {
        print!("\nfailures: ");
        println!(
            "    {}",
            failurecases
                .iter()
                .fold(String::new(), |s, per| s + "\n    " + per)
        );
    }

    if failurecases.len() == 0 {
        print!("\ntest result \x1B[1;32mok\x1B[0m. ");
    } else {
        print!("\ntest result \x1B[1;31mFAILED\x1B[0m. ");
    }

    println!(
        "{} passed; {} failed; {} ignored",
        npassed,
        failurecases.len(),
        nignored
    );
    failurecases.is_empty()
}

#[allow(clippy::print_literal)]
//pub fn test<F, R>(ncases: &mut u64, failurecases: &mut Vec<String>, f: F, name: &str)
//where
//    F: FnOnce() -> R + std::panic::UnwindSafe,
//pub fn test(ncases: &mut u64, failurecases: &mut Vec<String>, c: &TestCase) bool {
pub fn test(c: &TestCase) -> bool {
    use std::panic;
    use std::string::ToString;

    //*ncases += 1;

    let t = || {
        (c.func)();
    };

    if c.should_panic.is_none() {
        let ok = std::panic::catch_unwind(t).is_ok();
        if ok {
            println!("test {} ... \x1B[1;32mok\x1B[0m", c.id);
        } else {
            println!("test {} ... \x1B[1;31mFAILED\x1B[0m", c.id);
            //failurecases.push(c.id.clone());
        }

        return ok;
    }

    // suppress panicking for should_panic
    let panicker_backup = panic::take_panic_handler();
    // @TODO: figure if it's possible to pass in something into this fn pointer
    panic::set_panic_handler(|_| {});

    let expected = c.should_panic.as_ref().unwrap();
    let got = match std::panic::catch_unwind(t) {
        Ok(_) => Some("missing panic".to_string()),
        Err(err) => {
            let mut got = None;
            let done = match err.downcast_ref::<&str>() {
                Some(v) if v.contains(expected) => true,
                Some(v) => {
                    got = Some(v.to_string());
                    true
                }
                None => false,
            };

            let done = done
                || match err.downcast_ref::<String>() {
                    Some(v) if v.contains(expected) => true,
                    Some(v) => {
                        got = Some(v.to_string());
                        true
                    }
                    None => false,
                };

            if !done {
                got = Some("crate testing has missed your IMPORTANT edge case!!!!".to_string());
            }

            got
        }
    };

    let ok = if let Some(msg) = got {
        println!("test {} ... \x1B[1;31mFAILED\x1B[0m", c.id);
        println!(
            r#"    note: panic did not contain expected string
          panic message: `"{}"`
     expected substring: `"{}"`
"#,
            msg, expected
        );
        false
    } else {
        println!("test {} ... \x1B[1;32mok\x1B[0m", c.id);
        true
    };

    panic::set_panic_handler(panicker_backup);

    ok
}
