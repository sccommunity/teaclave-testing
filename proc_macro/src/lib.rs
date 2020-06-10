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

/// test parses the test function in pattern of
///
/// #[test]
/// #[should_panic(expected = "...")]
/// #[ignore]
/// fn any_fn_name_your_like() {
///     ...
/// }
///
/// @dev the order of #[should_panic] and #[ignore] is interchangeable.
/// @dev #[should_panic] and #[ignore] must comes after #[test] but not before
#[proc_macro_attribute]
pub fn test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let tokens = input.to_string();

    let (attrs, func) = match tokens.find("\nfn") {
        Some(v) => tokens.split_at(v),
        None => ("", tokens.as_str()),
    };
    let attrs = canonicalize_attributes(attrs);
    let attrs = attrs.as_str();

    let token_fn: TokenStream = func.parse().expect("invalid 'fn' token stream");

    let (should_panic, ignored) = figure_out_should_panic_and_ignored(attrs);
    let should_panic = if let Some(expected) = should_panic {
        quote! { Some(#expected) }
    } else {
        quote! { None }
    };

    let with_testing_gate = quote! { #[cfg(feature = "with-testing")] };

    let f = parse_macro_input!(token_fn as ItemFn);
    let f_ident = &f.sig.ident;
    // I know no ways to make the line/column for panic number right =_=
    // even if adding back original attributes to occupy lines
    let q = quote!(
        #with_testing_gate

        #f

        #with_testing_gate

        inventory::submit!(
            testing::TestCase::new(
                concat!(module_path!(), "::", stringify!(#f_ident)),
                #f_ident,
                #should_panic,
                #ignored,
            )
        );
    );

    //println!("{}", q);

    q.into()
}

// canonicalize_attributes canonicalized special characters in attrs.
// Currently, following replacements are taken
//   \" => "
fn canonicalize_attributes(attrs: &str) -> String {
    attrs.replace(r#"\""#, r#"""#)
}

fn figure_out_should_panic_and_ignored(attrs: &str) -> (Option<&str>, bool) {
    const SHOULD_PANIC: &str = r#"#\[should_panic(\(expected\s*=\s*"((?s).*)"\))?\]"#;
    const IGNORE: &str = r"#\[ignore\]";

    {
        // case: #[ignore] follows #[should_panic(*)]
        let should_panic_then_ignore = format!(r"^{}\s*({})?$", SHOULD_PANIC, IGNORE);
        let pattern = Regex::new(&should_panic_then_ignore).unwrap();

        if let Some(groups) = pattern.captures(attrs) {
            let should_panic_expected = groups.get(2).map_or("", |m| m.as_str());
            let ignored = groups.get(3).is_some();

            return (Some(should_panic_expected), ignored);
        }
    }

    {
        // case: #[should_panic(*)] follows #[ignore]
        let ignore_then_should_panic = format!(r"^{}\s*({})?$", IGNORE, SHOULD_PANIC);
        let pattern = Regex::new(&ignore_then_should_panic).unwrap();

        if let Some(groups) = pattern.captures(attrs) {
            let should_panic_expected = if groups.get(1).is_some() {
                Some(groups.get(2).map_or("", |m| m.as_str()))
            } else {
                None
            };

            return (should_panic_expected, true);
        }
    }

    (None, false)
}
