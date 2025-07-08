use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::{quote, ToTokens};

use crate::{block_table::{Block, BlockList}};

mod block_table;

#[proc_macro]
pub fn blocks(input: TokenStream) -> TokenStream {
    let BlockList { blocks } = parse_macro_input!(input as BlockList);

    let mut block_names = Vec::new(); 

    let mut next_index = 1usize;

    let mut consts = quote! {};
    let mut assertions = quote! {};

    for block in blocks {
        let Block { name, ty, expr } = block;

        block_names.push(name.clone());

        assertions.extend(quote! {
            static_assertions::assert_impl_all!(#ty: BlockType, BlockTypeBuildable);
        });

        let expr_with_index = quote! {
            (#expr).with_index(#next_index)
        };

        consts.extend(quote! {
            pub const #name: #ty = #expr_with_index;
        });

        let type_name = ty.to_token_stream().to_string();
        let type_size = match type_name.as_str() {
            "Basic" => 1,
            "Full" => 6,
            "Pillar" => 3,
            invalid_type => {
                return syn::Error::new_spanned(
                    &ty, 
                    format!("`{invalid_type}` must be a valid block.  Try Full, Basic, or Pillar.")
                ).to_compile_error().into();
            }
        };

        next_index += type_size;
    }

    let slice = quote! {
        pub static BLOCKS: &[&dyn BlockType] = &[
            #(&#block_names),*
        ];
    };

    let expanded = quote! {
        #assertions

        #consts

        #slice
    };

    expanded.into()
}