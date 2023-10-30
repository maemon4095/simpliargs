use proc_macro2::TokenStream;

pub fn simpliargs(attr: TokenStream, body: TokenStream) -> TokenStream {
    let item_struct: syn::ItemStruct = match syn::parse2(body) {
        Ok(e) => e,
        Err(e) => return e.to_compile_error(),
    };

    todo!()
}

/// ```
/// #[simpliargs(defaults = [single, optional, option])]
/// struct Args {
///     /// required option with single value
///     #[required, option("--single", "-s")]
///     option: usize, // field types must be implement Parse
///     /// required positional argument
///     #[positional(0)]
///     positional: String,
///     /// optional option with multiple values
///     /// multiple option require "FromIterator" implementation.
///     #[multiple, option]
///     multiple: Vec<String> // --multiple
///     /// optional option without default attribute require "Default" implementation
///     #[option("--optional")]    
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
///     #[multiple, positional]
///     rest_arguments: Vec<String>
/// }
///
/// #[simpliargs]
/// enum ArgsWithSubcommand {
///     #[simpliargs]
///     Choice0 {
///         /// struct like enum member is same for struct
///     },
///     #[subcommand("c")]
///     Choice1(A),
///     Choice2 // choice2
/// }
/// ```
enum GenerationOptions {
    Subcommand,
    Option,
}
