extern crate common;
extern crate devise;
extern crate proc_macro2;
#[macro_use] extern crate quote;
extern crate syn;

mod instruction_macros;

use instruction_macros::{build_instruction_macro, instruction_macro, opcode_variations_macro};
use proc_macro::TokenStream;

#[proc_macro]
pub fn build_instruction(item: TokenStream) -> TokenStream {
    match build_instruction_macro(proc_macro2::TokenStream::from(item)) {
        Ok(output) => output.into(),
        Err(e) => panic!("{}", e),
    }
}

#[proc_macro_attribute]
pub fn instruction(args: TokenStream, item: TokenStream) -> TokenStream {
    match instruction_macro(proc_macro2::TokenStream::from(args), proc_macro2::TokenStream::from(item)) {
        Ok(output) => output.into(),
        Err(e) => panic!("{}", e),
    }
}

#[proc_macro]
pub fn opcode_variations(item: TokenStream) -> TokenStream {
    match opcode_variations_macro(proc_macro2::TokenStream::from(item)) {
        Ok(output) => output.into(),
        Err(e) => panic!("{}", e),
    }
}

// #[proc_macro]
// pub fn run_instruction(item: TokenStream) -> TokenStream {
//     match run_instruction_macro(proc_macro2::TokenStream::from(item)) {
//         // Ok(output) => output.into(),
//         Ok(output) => output.into(),
//         Err(e) => panic!("{}", e),
//     }
// }