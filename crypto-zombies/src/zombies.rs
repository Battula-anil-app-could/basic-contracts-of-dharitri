
dharitri_sc::imports!();
dharitri_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopDecode, TopEncode, TypeAbi)]
pub struct Zombie<M: ManagedTypeApi>{
    pub name: ManagedBuffer<M>,
    pub dna: u64,
    
}