use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// macro_rules! 是基于文本的宏，在代码的文本级别上进行替换，生成的代码在编译时展开，
// 而 procedural macros 是通过在编译期间执行 Rust 代码来生成、转换或修改 Rust 代码的宏。

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}

#[warn(unused_macros)]
macro_rules! emit {
    ($tokens:expr) => {
        // 该 trait 提供了处理 Rust 编译器诊断信息的扩展方法
        use devise::ext::SpanDiagnosticExt;
        let mut tokens = $tokens;

        // std::env::var_os("ROCKET_CODEGEN_DEBUG").is_some() 是一个 Rust 的标准库 std::env 模块中的函数调用，用于检查环境变量 "ROCKET_CODEGEN_DEBUG" 是否存在，并返回一个 Option 类型的值。
        if std::env::var_os("ROCKET_CODEGEN_DEBUG").is_some() {
            let debug_tokens = proc_macro2::Span::call_site()
            .note("emitting Rocket code generation debug output")
            .note(tokens.to_string())
            .emit_as_item_tokens();

            tokens.extend(debug_tokens);
        }

        // 将 tokens 转换为表达式的返回值，从而完成了整个宏的实现。
        tokens.into()
    };
}

// #[proc_macro_derive(FromFormField)]
// pub fn derive_from_form_field(input: TokenStream) -> TokenStream {
// 
// }
