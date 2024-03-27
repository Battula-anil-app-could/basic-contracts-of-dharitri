use crate::{storage, zombies::Zombie};


dharitri_sc::imports!();
dharitri_sc::derive_imports!();


#[dharitri_sc::module]
pub trait ZombieFactory: storage::StroageModule{
    fn create_zombie(&self, owner:ManagedAddress, name: ManagedBuffer, dna: u64){
        self.zombie_last_index().update(|id|{
            self.new_zombie_event(*id, name.clone(), dna);
            let new_zombie = Zombie{name, dna};
            self.owned_zombies(&owner).insert(*id);
            self.zombie_owner(*id).set(owner);
            self.zombies(*id).set(new_zombie);
            
            *id += 1usize;
        })
    }
    #[view]
    fn generate_random_dna(&self) -> u64{
        let mut my_random = RandomnessSource::new();
        let dna_digits = self.dna_digits().get();
        let max_dna_value = u64::pow(10u64, dna_digits as u32);
        my_random.next_u64_in_range(0u64, max_dna_value)
    }

    #[endpoint]
    fn create_random_zombie(&self, name: ManagedBuffer){
        let caller = self.blockchain().get_caller();
        require!(self.owned_zombies(&caller).is_empty(), "you already own a zombie");
        let dna = self.generate_random_dna();
        self.create_zombie(caller, name, dna);
    }

    #[event("newZombieEvent")]
    fn new_zombie_event(&self, #[indexed] zombie_id: usize, name: ManagedBuffer, #[indexed] dna: u64);
}