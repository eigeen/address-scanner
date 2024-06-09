use address::memory_utils;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, LitStr};

mod internal;
mod address_record;

#[proc_macro]
/// 将特征码模板字符串字面量转换为字节数组
///
/// Input:
///
/// ```
/// "F3 48 0F 2A F0 85 ** 7E ** 49 8B ** ** ** 00 00 ** C0 48 85 ** 74"
/// ```
///
/// Output:
///
/// ```
/// [0xF3, 0x48, 0x0F, 0x2A, 0xF0, 0x85, 0xFF, 0x7E, 0xFF, 0x49, 0x8B, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xC0, 0x48, 0x85, 0xFF, 0x74]
/// ```
///
/// 通配符：支持 `**`, `??`, `?`，通配符转换为 `0xFF`
pub fn hex_str_to_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_string = parse_macro_input!(input as LitStr).value();
    let bytes = memory_utils::space_hex_to_bytes(&input_string).unwrap();
    let bytes_ref: &[u8] = bytes.as_ref();

    let output = quote! {
        [ #(#bytes_ref),* ]
    };

    output.into_token_stream().into()
}

#[proc_macro_derive(AddressRecord, attributes(record))]
pub fn derive_address_record_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    address_record::derive_address_record_impl(input)
}

