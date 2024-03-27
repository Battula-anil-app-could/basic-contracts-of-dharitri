#![no_std]

mod zombies;
mod zombie_factory;
mod storage;
mod zombie_feading;
mod crypto_kitty_proxy;


dharitri_sc::imports!();
dharitri_sc::derive_imports!();


#[dharitri_sc::contract]
pub trait CryptoZombies: zombie_factory::ZombieFactory + storage::StroageModule + zombie_feading::ZombieFeading{
    #[init]
    fn init(&self) {
        self.dna_digits().set(16u8);
        self.zombie_last_index().set(1usize);
    }

    
   
}
