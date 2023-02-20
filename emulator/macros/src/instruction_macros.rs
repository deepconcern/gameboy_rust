use std::fmt::{Display, Debug};

use common::opcode::Opcode;
use devise::{FromMeta, Diagnostic};
use proc_macro2::TokenStream;
use syn::{parse2, ItemFn, Ident};

#[derive(FromMeta)]
struct Attribute {
    pub cycles: usize,
    pub name: String,
    pub opcode_pattern: String,
}

#[derive(Debug)]
pub enum InstructionMacroError {
    Diagnostic(Diagnostic),
    SynError(syn::Error),
}

impl Display for InstructionMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionMacroError::Diagnostic(d) => write!(f, "(Diagnostic error): {:?}", d),
            InstructionMacroError::SynError(s) => write!(f, "(Parsing error): {}", s),
        }
    }
}

impl From<Diagnostic> for InstructionMacroError {
    fn from(value: Diagnostic) -> Self {
        InstructionMacroError::Diagnostic(value)
    }
}

impl From<syn::Error> for InstructionMacroError {
    fn from(value: syn::Error) -> Self {
        InstructionMacroError::SynError(value)
    }
}

pub type InstructionMacroResult = Result<TokenStream, InstructionMacroError>;

struct Instruction {
    pub cycles: usize,
    pub name: String,
    pub operation: ItemFn,
    pub opcode: Opcode,
}

impl Instruction {
    pub fn from (attribute: Attribute, operation: ItemFn) -> Instruction {
        let trimmed_opcode_pattern = attribute.opcode_pattern.replace(" ", "");

        if trimmed_opcode_pattern.len() != 8 {
            panic!("invalid opcode (must be 8 bits)");
        }

        Instruction {
            cycles: attribute.cycles,
            name: attribute.name,
            operation,
            opcode: Opcode::new(&attribute.opcode_pattern),
        }
    }
}

pub fn build_instruction_macro(item: TokenStream) -> InstructionMacroResult {
    let arg: Ident = parse2(item.into())?;

    let tokens = quote!(#arg {});

    Ok(tokens)
}

pub fn instruction_macro(args: TokenStream, item: TokenStream) -> InstructionMacroResult {
    let attribute_tokens = quote!(instruction(#args));
    let full_attribute = Attribute::from_meta(&parse2(attribute_tokens.into())?)?;
    let operation = parse2(item.clone().into())?;

    let instruction = Instruction::from(full_attribute, operation);

    let cycles = instruction.cycles;
    let name = instruction.name;
    let opcode = instruction.opcode;
    let operation_body = &instruction.operation.block;
    let operation_name = &instruction.operation.sig.ident;
    let vis = &instruction.operation.vis;

    let variations = opcode.variations;

    let tokens = quote! {
        #[allow(non_camel_case_types)]
        #vis struct #operation_name { }

        impl Instruction for #operation_name {
            fn cycles(&self) -> usize {
                #cycles
            }
        
            fn name(&self) -> String {
                String::from(#name)
            }
        
            fn operation(&self, processor_state: &mut ProcessorState, opcode: u8) -> Result<(), InstructionError> {
                #operation_body
            }

            fn variations(&self) -> Vec<u8> {
                vec![#(#variations),*]
            }
        }
    };

    Ok(tokens)
}

pub fn opcode_variations_macro(item: TokenStream) -> InstructionMacroResult {
    let instruction_struct: Ident = parse2(item.into())?;

    Ok(quote!((#instruction_struct {}).variations()))
}

// pub fn run_instruction_macro(item: TokenStream) -> InstructionMacroResult {
//     let tokens = quote!(opcode_variations(#item));

//     let full_function: syn::ExprCall = parse2(tokens.into())?;

//     let full_args = full_function.args.into_iter().collect::<Vec<syn::Expr>>();

//     let instruction = &full_args[0];
//     let processor_state = &full_args[1];
//     let opcode = &full_args[2];

//     let tokens = quote!((#instruction {}).operation(#processor_state, #opcode));

//     Ok(tokens)
// }