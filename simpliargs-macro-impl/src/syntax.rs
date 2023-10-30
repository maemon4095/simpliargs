use syn::{
    parse::Parse,
    punctuated::{self, Punctuated},
    spanned::Spanned,
    token::Brace,
    Attribute, Expr, Field, Lit, LitStr, Token, Visibility,
};

pub struct GenerationOptionWithArgs {
    pub ident: syn::Ident,
    pub paren_token: syn::token::Paren,
    pub args: Punctuated<GenerationOption, Token![,]>,
}

pub enum GenerationOption {
    Expr(Expr),
    WithArgs(GenerationOptionWithArgs),
    Ident(syn::Ident),
}

pub struct FieldsNamed {
    brace_token: syn::token::Brace,
    named: Punctuated<Field, syn::Token![,]>,
}
pub struct FieldsUnamed {}
pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsNamed),
    Unit,
}

struct GenTargetStruct {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub struct_token: syn::Token![struct],
    pub ident: syn::Ident,
    pub fields: Fields,
    pub semi_token: syn::Token![;],
}
struct GenTargetEnum {}

enum GenTarget {
    Struct(GenTargetStruct),
    Enum(GenTargetEnum),
}
