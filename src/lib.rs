extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

/// Implements `std::convert::Into<Box<dyn T>>` for one or more traits.
///
/// # Usage:
/// ```rust
/// use derive_box_dyn::IntoBoxDyn;
///
/// #[derive(IntoBoxDyn)]
/// #[box_dyn_traits(MyTrait, MyOtherTrait)]
/// struct MyStruct;
///
/// trait MyTrait{}
/// trait MyOtherTrait{}
///
/// impl MyTrait for MyStruct {}
/// impl MyOtherTrait for MyStruct {}
///
/// let _: Box<dyn MyTrait> = MyStruct {}.into();
/// let _: Box<dyn MyOtherTrait> = MyStruct {}.into();
/// ```
#[proc_macro_derive(IntoBoxDyn, attributes(box_dyn_traits))]
pub fn derive_into_box_dyn(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as ItemStruct);
    let struct_name = ast.ident.to_string();
    let target_traits = get_target_traits(ast);

    if target_traits.len() == 0 {
        panic!("box_dyn_traits attribute must be defined to derive IntoBoxDyn")
    }

    target_traits
        .iter()
        .map(|target_trait| template_impl(&struct_name, target_trait))
        .collect::<Vec<String>>()
        .join("\n")
        .parse()
        .unwrap()
}

fn get_target_traits(ast: ItemStruct) -> Vec<String> {
    ast.attrs
        .iter()
        .filter_map(|attr| match &attr.meta {
            syn::Meta::List(l) => match l.path.segments.first() {
                Some(seg) => Some((&seg.ident, l)),
                _ => None,
            },
            _ => None,
        })
        .filter(|a| a.0 == "box_dyn_traits")
        .map(|a| {
            a.1.tokens
                .clone()
                .into_iter()
                .map(|t| t.to_string())
                .filter(|t| t != ",")
        })
        .flatten()
        .collect::<Vec<String>>()
}

// template_impl templates a single implementation of std::convert::Into.
fn template_impl(struct_name: &str, trait_name: &str) -> String {
    format!(
        "
impl std::convert::Into<Box<dyn {trait_name}>> for {struct_name} {{
    fn into(self) -> Box<dyn {trait_name}> {{
        Box::new(self)
    }}
}}"
    )
}
