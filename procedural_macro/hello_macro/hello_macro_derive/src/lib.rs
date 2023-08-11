use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

///
/// 宏所在的包名必须以derive为后缀，对于hello_macro宏而言，包名就应该是 hello_macro_derive(库包)
///
/// derive过程宏只能用在struct/enum/union上，多数用在结构体上
///
/// DeriveInput {
///     // 属性
///     attrs: Vec<Attribute>,
///     // 可视范围
///     vis: Visibility { // 是个枚举
///         Public(Token![pub]),
///         Restricted(VisRestricted),
///         Inherited,
///     },
///     // 泛型
///     generics: Generics {
///         lt_token: Option<Token![<]>,
///         params: Punctuated<GenericParam, Token![,]>,
///         gt_token: Option<Token![>]>,
///         where_clause: Option<WhereClause>,
///     },
///     // 标识符
///     ident: Ident {
///         ident: "Pancakes",
///         span: #0 bytes(95..103)
///     },
///     // 数据
///     data: Struct(
///         DataStruct {
///             struct_token: Struct,
///             fields: Unit,
///             semi_token: Some(
///                 Semi
///             )
///         }
///     )
/// }
///
/// quote! 宏能让我们编写希望返回的 Rust 代码
///
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast: DeriveInput = syn::parse(input).expect("Failed to parse input");

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
