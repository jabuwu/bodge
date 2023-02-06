use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Scenes)]
pub fn derive_scene(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let mut quotes = vec![];
    if let Data::Enum(enm) = input.data {
        for variant in enm.variants.iter() {
            let variant_ident = &variant.ident;
            quotes.push(quote! {
                app.add_system_set(bevy::ecs::schedule::SystemSet::on_exit(#ident::#variant_ident).with_system(bodge::bevy::cleanup_non_persistent_entities));
            });
        }
    }
    TokenStream::from(quote! {
        impl Scenes for #ident {
            fn init_scene(app: &mut bevy::app::App) {
                app.add_state(#ident::default());
                #(#quotes)*
            }
        }
    })
}

#[proc_macro_derive(IyesScenes)]
pub fn derive_iyes_scene(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let mut quotes = vec![];
    if let Data::Enum(enm) = input.data {
        for variant in enm.variants.iter() {
            let variant_ident = &variant.ident;
            quotes.push(quote! {
                app.add_exit_system(#ident::#variant_ident, bodge::bevy::cleanup_non_persistent_entities);
            });
        }
    }
    TokenStream::from(quote! {
        impl IyesScenes for #ident {
            fn init_scene(app: &mut bevy::app::App) {
                use iyes_loopless::prelude::AppLooplessStateExt;
                app.add_loopless_state(#ident::default());
                #(#quotes)*
            }
        }
    })
}
