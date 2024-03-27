// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            4
// Async Callback:                       1
// Total number of exported functions:   6

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    crypto_zombies
    (
        init => init
        generate_random_dna => generate_random_dna
        create_random_zombie => create_random_zombie
        feed_and_multiply => feed_and_multiply
        feed_on_kitty => feed_on_kitty
    )
}

dharitri_sc_wasm_adapter::async_callback! { crypto_zombies }
