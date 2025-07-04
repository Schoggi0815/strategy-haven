use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input};

#[derive(FromDeriveInput)]
#[darling(attributes(entity))]
struct Opts {
    entity: Ident,
    entity_db: Ident,
}

#[proc_macro_derive(SpacetimeChannelPlugin, attributes(entity))]
pub fn spacetime_channel_plugin(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");

    // Build the trait implementation.
    impl_spacetime_channel_plugin(&input, opts)
}

fn impl_spacetime_channel_plugin(input: &syn::DeriveInput, opts: Opts) -> TokenStream {
    let name = &input.ident;
    let entity = &opts.entity;

    let generated = quote! {
        impl bevy::prelude::Plugin for #name {
            fn build(&self, app: &mut App) {
                app.init_resource::<crate::spacetime_db::spacetime_channel::SpacetimeChannel<crate::module_bindings::#entity>>()
                    .add_systems(
                        OnEnter(crate::spacetime_db::spacetime_state::SpacetimeState::Initialized),
                        Self::register_channels,
                    )
                    .add_systems(
                        Update,
                        (
                            crate::spacetime_db::spacetime_channel::write_insert_event::<crate::module_bindings::#entity>,
                            crate::spacetime_db::spacetime_channel::write_update_event::<crate::module_bindings::#entity>,
                            crate::spacetime_db::spacetime_channel::write_delete_event::<crate::module_bindings::#entity>,
                        ),
                    )
                    .add_event::<crate::spacetime_db::spacetime_channel::InsertEvent<crate::module_bindings::#entity>>()
                    .add_event::<crate::spacetime_db::spacetime_channel::UpdateEvent<crate::module_bindings::#entity>>()
                    .add_event::<crate::spacetime_db::spacetime_channel::DeleteEvent<crate::module_bindings::#entity>>();
            }
        }
    };
    generated.into()
}

#[proc_macro_derive(SpacetimeChannelRegisterer, attributes(entity))]
pub fn spacetime_channel_registerer(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");

    // Build the trait implementation.
    impl_spacetime_channel_registerer(&input, opts)
}

fn impl_spacetime_channel_registerer(input: &syn::DeriveInput, opts: Opts) -> TokenStream {
    let name = &input.ident;
    let entity = &opts.entity;
    let entity_db = &opts.entity_db;

    let generated = quote! {
        impl crate::spacetime_db::spacetime_channel::SpacetimeChannelRegisterer<crate::module_bindings::#entity> for #name {
            fn register_channels(
                channel: Res<SpacetimeChannel<crate::module_bindings::#entity>>,
                spacetime: Res<crate::spacetime_db::spacetime_server::ServerConnection>,
            ) {
                let insert_sender = channel.insert_sender.clone();
                let delete_sender = channel.delete_sender.clone();

                spacetime.0.db.#entity_db().on_insert(move |_ctx, entity| {
                    insert_sender
                        .try_send(entity.clone())
                        .expect("Unbounded channel should never block!");
                });

                spacetime.0.db.#entity_db().on_delete(move |_ctx, entity| {
                    delete_sender
                        .try_send(entity.clone())
                        .expect("Unbounded channel should never block!");
                });
            }
        }
    };
    generated.into()
}

#[proc_macro_derive(SpacetimeUpdateChannelRegisterer, attributes(entity))]
pub fn spacetime_update_channel_registerer(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");

    // Build the trait implementation.
    impl_spacetime_update_channel_registerer(&input, opts)
}

fn impl_spacetime_update_channel_registerer(input: &syn::DeriveInput, opts: Opts) -> TokenStream {
    let name = &input.ident;
    let entity = &opts.entity;
    let entity_db = &opts.entity_db;

    let generated = quote! {
        impl crate::spacetime_db::spacetime_channel::SpacetimeChannelRegisterer<crate::module_bindings::#entity> for #name {
            fn register_channels(
                channel: Res<SpacetimeChannel<crate::module_bindings::#entity>>,
                spacetime: Res<crate::spacetime_db::spacetime_server::ServerConnection>,
            ) {
                let update_sender = channel.update_sender.clone();
                let insert_sender = channel.insert_sender.clone();
                let delete_sender = channel.delete_sender.clone();

                spacetime.0.db.#entity_db().on_insert(move |_ctx, entity| {
                    insert_sender
                        .try_send(entity.clone())
                        .expect("Unbounded channel should never block!");
                });

                spacetime.0.db.#entity_db().on_delete(move |_ctx, entity| {
                    delete_sender
                        .try_send(entity.clone())
                        .expect("Unbounded channel should never block!");
                });

                spacetime.0.db.#entity_db().on_update(move |_ctx, _old, entity| {
                    update_sender
                        .try_send(entity.clone())
                        .expect("Unbounded channel should never block!");
                });
            }
        }
    };
    generated.into()
}
