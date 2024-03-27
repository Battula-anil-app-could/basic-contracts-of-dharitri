dharitri_sc::imports!();
dharitri_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopDecode, TopEncode, TypeAbi)]
pub struct Kitty{
    pub is_gestating: bool,
    pub is_ready: bool,
    pub cooldown_index: u64,
    pub next_action_at: u64,
    pub string_with_id: u64,
    pub birth_time: u64,
    pub natron_id: u64,
    pub sire_id: u64,
    pub generation: u64,
    pub genes: u64,
}


#[dharitri_sc::proxy]
pub trait CryptoKitties{
    #[endpoint]
    fn get_kitty(&self, id: usize) -> Kitty;

}