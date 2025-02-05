use crate::crypto_kitty_proxy::Kitty;
use crate::storage;
use crate::zombie_factory;
use crate::crypto_kitty_proxy;
dharitri_sc::imports!();
dharitri_sc::derive_imports!();


#[dharitri_sc::module]
pub trait ZombieFeading: storage::StroageModule + zombie_factory::ZombieFactory{

    #[endpoint]
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64){
        let caller = self.blockchain().get_caller();
        require!(caller == self.zombie_owner(zombie_id).get(), "Only the owner of the zombie can do this operation");

        let my_zombie = self.zombies(zombie_id).get();

        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);

        let verified_target_dna = target_dna % max_dna_value;
        let new_dna = (my_zombie.dna + verified_target_dna) / 2;
        let new_name = ManagedBuffer::from(b"NoName");
        self.create_zombie(caller, new_name, new_dna);
    }

    #[callback]
    fn get_kitty_callback(&self, #[call_result] result: ManagedAsyncCallResult<Kitty>, zombie_id: usize){

        match result {
            ManagedAsyncCallResult::Ok(kitty) => {
                let kitty_dna = kitty.genes;
                self.feed_and_multiply(zombie_id, kitty_dna);
            },
            ManagedAsyncCallResult::Err(_) => {},
        }
    }

    #[endpoint]
    fn feed_on_kitty(&self, zombie_id: usize, kitty_id: usize){
        let crypto_kitties_sc_address = self.crypto_kitties_sc_address().get();
        self.kitty_proxy(crypto_kitties_sc_address)
        .get_kitty(kitty_id)
        .async_call()
        .with_callback(self.callbacks().get_kitty_callback(zombie_id))
        .call_and_exit();
    }

    #[proxy]
    fn kitty_proxy(&self, to: ManagedAddress) -> crypto_kitty_proxy::Proxy<Self::Api>;

}