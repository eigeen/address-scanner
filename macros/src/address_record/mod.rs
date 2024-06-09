use core::panic;

use convert_case::{Case, Casing};
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

use crate::internal::symbols::*;

pub fn derive_address_record_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut pattern: Option<String> = None;
    let mut offset: Option<i64> = None;

    for attr in input.attrs.iter() {
        if attr.path() != RECORD {
            continue;
        }

        if let Err(e) = attr.parse_nested_meta(|meta| {
            if meta.path == PATTERN {
                let expr: syn::Expr = meta.value()?.parse()?;
                let mut value = &expr;
                while let syn::Expr::Group(e) = value {
                    value = &e.expr;
                }
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = value
                {
                    let suffix = lit.suffix();
                    if !suffix.is_empty() {
                        panic!("unexpected suffix `{}` on string literal", suffix);
                    }
                    pattern = Some(lit.value());
                } else {
                    panic!("pattern attribute must be a string literal");
                }
                return Ok(());
            }
            if meta.path == OFFSET {
                let expr: syn::Expr = meta.value()?.parse()?;
                let mut value = &expr;

                while let syn::Expr::Group(e) = value {
                    value = &e.expr;
                }

                let mut is_neg = false;
                if let syn::Expr::Unary(syn::ExprUnary { attrs: _, op, expr }) = value {
                    if let syn::UnOp::Neg(_) = op {
                        value = expr;
                        is_neg = true;
                    } else {
                        panic!("offset attribute must be a valid negative integer literal")
                    }
                }

                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit),
                    ..
                }) = value
                {
                    let mut num = lit.base10_parse::<i64>().unwrap();
                    if is_neg {
                        num = -num;
                    }
                    offset = Some(num);
                } else {
                    panic!("offset attribute must be a integer literal")
                }

                return Ok(());
            }
            Ok(())
        }) {
            panic!("Failed to parse pattern attribute: {e}");
        };
        continue;
    }

    if pattern.is_none() {
        panic!("Pattern attribute is required")
    }
    if offset.is_none() {
        offset.replace(0);
    }

    let const_pattern = Ident::new(
        &format!(
            "CONST_PATTERN_STR_{}",
            input.ident.to_string().to_case(Case::UpperSnake)
        ),
        Span::call_site(),
    );
    let const_offset = Ident::new(
        &format!(
            "CONST_OFFSET_{}",
            input.ident.to_string().to_case(Case::UpperSnake)
        ),
        Span::call_site(),
    );
    let consts = quote! {
        pub const #const_pattern: &str = #pattern;
        pub const #const_offset: i64 = #offset;
    };
    let struct_name = input.ident;
    let mod_name = Ident::new(
        &format!("__private_{}", struct_name.to_string().to_case(Case::Snake)),
        Span::call_site(),
    );

    let output = quote! {
        mod #mod_name {
            #consts
        }

        impl ::address_scanner::AddressProvider for #struct_name {
            fn get_address(&self) -> Result<u64, ::address_scanner::Error> {
                Self::scan_first()
            }
        }

        impl #struct_name {
            pub fn scan_first() -> Result<u64, ::address_scanner::Error> {
                let result = ::address_scanner::MemoryUtils::scan_first(#mod_name::#const_pattern);
                result.map(|address| {
                    (address as i64 + #mod_name::#const_offset) as u64
                }).map_err(::address_scanner::Error::Memory)
            }
        }
    };

    // panic!("{}", output);
    output.into()
}
