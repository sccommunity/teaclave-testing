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

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let tokens = input.to_string();

    let mut lines: Vec<&str> = tokens.split('\n').collect();
    if lines.len() > 2 {
        panic!("only #[should_panic(*)] and fn is supported now");
    }

    let token_fn: TokenStream = lines
        .pop()
        .expect("missing fn")
        .parse()
        .expect("invalid fn token stream");

    //let other_attrs: Vec<proc_macro2::TokenStream> = lines.iter().map(|v| v.parse().unwrap()).collect();

    let should_panic = if lines.len() != 0 {
        let r = Regex::new(r#"#\[should_panic(\(expected\s*=\s*"(.*)"\))?\]"#).unwrap();
        // @TODO: figure out the actual meaning of groups.len()
        let expected = r
            .captures(lines[0])
            .expect("invalid #[should_panic] attribute")
            .get(2)
            .map_or("", |m| m.as_str());

        quote! { Some(#expected) }
    } else {
        quote! { None }
    };

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
            )
        );
    );

    q.into()
}
