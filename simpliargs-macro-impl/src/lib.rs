mod syntax;
use proc_macro2::TokenStream;

/// ```
/// #[simpliargs(help("-h", "--help"), defaults(single, optional, flag))]
/// struct Args {
///     /// required option with single value
///     #[required, flag("--single", "-s")]
///     option: usize, // field types must be implement simpliargs::Parse
///
///     /// required positional argument
///     #[positional(0)]
///     positional: String,
///
///     /// optional option with multiple values
///     /// multiple option require "FromIterator" implementation.
///     #[multiple, flag]
///     multiple: Vec<String> // --multiple
///
///     /// optional option without default attribute require "Default" implementation
///     #[flag("--optional")]
///     optional: Option<String>
///
///     /// explicit optional
///     #[optional, default(1)]
///     explicit_optional: usize,
///
///     /// explicit single
///     #[single]
///     explicit_optional: usize,
///
///     default_field: String // --default_field
///
///     /// subcommand
///     #[flag("sub")]
///     subcommand: Option<ArgsWithSubcommand>,
///
///     #[multiple, positional]
///     rest_arguments: Vec<String>
/// }
///
/// #[simpliargs(nohelp)]
/// enum ArgsWithSubcommand {
///     #[flag, simpliargs]
///     Choice0 {
///         /// struct like enum member is same for struct
///     },
///     #[flag("c")]
///     Choice1(A),
///     Choice2 // choice2
/// }
/// ```
pub fn simpliargs(attr: TokenStream, body: TokenStream) -> TokenStream {
    let _err_not_struct = match syn::parse2(body.clone()) {
        Ok(e) => return gen_struct(e),
        Err(e) => e,
    };

    let _err_not_enum = match syn::parse2(body.clone()) {
        Ok(e) => return gen_enum(e),
        Err(e) => e,
    };

    syn::Error::new_spanned(body, "simpliargs only supports struct or enum.").into_compile_error()
}

fn gen_struct(item_struct: syn::ItemStruct) -> TokenStream {
    todo!()
}

fn gen_enum(item_enum: syn::ItemEnum) -> TokenStream {
    todo!()
}
