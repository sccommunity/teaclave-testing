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

extern crate proc_macro;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

use proc_macro::TokenStream;
use regex::Regex;

#[cfg(test)]
mod tests;

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let tokens = input.to_string();

    let mut lines: Vec<&str> = tokens.split('\n').collect();
    if lines.len() > 3 {
        panic!("only '#[should_panic(*)]', '#[ignore]' and fn is supported now");
    }

    let token_fn: TokenStream = lines
        .pop()
        .expect("missing fn")
        .parse()
        .expect("invalid fn token stream");

    let (should_panic, ignored) = figure_out_should_panic_and_ignored(&lines);

    let f = parse_macro_input!(token_fn as ItemFn);
    let f_ident = &f.sig.ident;
    // I know no ways to make the line/column for panic number right =_=
    // even if adding back original attributes to occupy lines
    let q = quote!(
        #f

        inventory::submit!(
            testing::TestCase::new(
                concat!(module_path!(), "::", stringify!(#f_ident)),
                #f_ident,
                #should_panic,
                #ignored,
            )
        );
    );

    q.into()
}

fn figure_out_should_panic_and_ignored(lines: &Vec<&str>) -> (proc_macro2::TokenStream, bool) {
    let regex_should_panic =
        Regex::new(r#"#\[should_panic(\(expected\s*=\s*"(.*)"\))?\]"#).unwrap();
    const REGEX_IGNORED: &'static str = "#[ignore]";

    let mut should_panic = None;
    let mut ignored = false;
    for line in lines {
        if line == &REGEX_IGNORED {
            ignored = !ignored || panic!("duplicate #[ignore]");
            continue;
        }

        if should_panic.is_some() {
            panic!("duplicate #[should_panic] or unsupported attributes");
        }

        // @TODO: figure out the actual meaning of groups.len()
        if let Some(groups) = regex_should_panic.captures(line) {
            let expected = groups.get(2).map_or("", |m| m.as_str());
            should_panic = Some(expected);
        }
    }

    let should_panic = if let Some(expected) = should_panic {
        quote! { Some(#expected) }
    } else {
        quote! { None }
    };

    (should_panic, ignored)
}
