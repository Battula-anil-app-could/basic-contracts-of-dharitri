use crate::zombies::Zombie;



dharitri_sc::imports!();
dharitri_sc::derive_imports!();


#[dharitri_sc::module]
pub trait StroageModule{
    #[storage_mapper("dnaDigits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("zombieLastIndex")]
    fn zombie_last_index(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("zombies")]
    fn zombies(&self, id: usize) -> SingleValueMapper<Zombie<Self::Api>>;

    #[storage_mapper("owendZombies")]
    fn owned_zombies(&self, owner: &ManagedAddress) -> UnorderedSetMapper<usize>;

    #[storage_mapper("zombieOwner")]
    fn zombie_owner(&self, id: usize) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("cryptoKittiesScAddress")]
    fn crypto_kitties_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}