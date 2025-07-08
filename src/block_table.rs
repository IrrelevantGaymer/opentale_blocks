use syn::{parse::{Parse, ParseStream}, Expr, Ident, Token, Type};

pub(crate) struct Block {
    pub(crate) name: Ident,
    pub(crate) ty: Type,
    pub(crate) expr: Expr,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        let expr: Expr = input.parse()?;
        input.parse::<Token![;]>()?;

        Ok(Block {name, ty, expr})
    }
}

pub(crate) struct BlockList {
    pub(crate) blocks: Vec<Block>
}

impl Parse for BlockList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut blocks = Vec::new();
        while !input.is_empty() {
            blocks.push(input.parse()?);
        }
        Ok(BlockList { blocks })
    }
}