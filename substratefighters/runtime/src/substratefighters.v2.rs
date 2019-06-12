use support::{decl_storage, decl_module, StorageValue, StorageMap,
    dispatch::Result, ensure, decl_event, traits::Currency};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash, Zero};
use parity_codec::{Encode, Decode};
use rstd::cmp;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FighterV1<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
    strength: u64,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FighterV2<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
    strength: u64,
    wins: u64,
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

type CurrentFighterVersion<T, U> = FighterV2<T, U>;
type CurrentFightersStorage<T> = FightersV2<T>;

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as balances::Trait>::Balance
    {
        Created(AccountId, Hash),
        PriceSet(AccountId, Hash, Balance),
        Transferred(AccountId, AccountId, Hash),
        Bought(AccountId, AccountId, Hash, Balance),
        VersionUpdated(u64),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as FighterStorage {
        FightersV1: map T::Hash => FighterV1<T::Hash, T::Balance>;
        FightersV2 get(fighter): map T::Hash => FighterV2<T::Hash, T::Balance>;

        FighterOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllFightersArray get(fighter_by_index): map u64 => T::Hash;
        AllFightersCount get(all_fighters_count): u64;
        AllFightersIndex: map T::Hash => u64;

        OwnedFightersArray get(fighter_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedFightersCount get(owned_fighter_count): map T::AccountId => u64;
        OwnedFightersIndex: map T::Hash => u64;
        
        Nonce: u64;
        Version get(version): u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event<T>() = default;

        fn on_initialize() {
            if Self::version() == 0 {
                for i in 0..Self::all_fighters_count() {
                    let fighter_hash = Self::fighter_by_index(i);
                    let fighter = <FightersV1<T>>::take(fighter_hash);

                    let fighter_new = FighterV2 {
                        id: fighter.id,
                        dna: fighter.dna,
                        price: fighter.price,
                        strength: fighter.strength,
                        wins: 0,
                    };

                    <FightersV2<T>>::insert(fighter_hash, fighter_new);
                }

                <Version<T>>::put(2);
                Self::deposit_event(RawEvent::VersionUpdated(2));
            }
        }

        fn create_fighter(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let nonce = <Nonce<T>>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            let strength = random_hash.as_ref()[3];

            let new_fighter = CurrentFighterVersion {
                id: random_hash,
                dna: random_hash,
                price: <T::Balance as As<u64>>::sa(0),
                strength: strength.into(),
                wins: 0,
            };

            Self::mint(sender, random_hash, new_fighter)?;
            
            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }

        fn set_price(origin, fighter_id: T::Hash, new_price: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(<CurrentFightersStorage<T>>::exists(fighter_id), "This cat does not exist");

            let owner = Self::owner_of(fighter_id).ok_or("No owner for this Fighter")?;
            ensure!(owner == sender, "You do not own this cat");

            let mut fighter = Self::fighter(fighter_id);
            fighter.price = new_price;

            <CurrentFightersStorage<T>>::insert(fighter_id, fighter);

            Self::deposit_event(RawEvent::PriceSet(sender, fighter_id, new_price));

            Ok(())
        }
        
        fn transfer(origin, to: T::AccountId, fighter_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;

            let owner = Self::owner_of(fighter_id).ok_or("No owner for this Fighter")?;
            ensure!(owner == sender, "You do not own this Fighter");

            Self::transfer_from(sender, to, fighter_id)?;

            Ok(())
        }

        fn buy_fighter(origin, fighter_id: T::Hash, max_price: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(<CurrentFightersStorage<T>>::exists(fighter_id), "This cat does not exist");

            let owner = Self::owner_of(fighter_id).ok_or("No owner for this Fighter")?;
            ensure!(owner != sender, "You can't buy your own cat");

            let mut fighter = Self::fighter(fighter_id);

            let fighter_price = fighter.price;
            ensure!(!fighter_price.is_zero(), "The cat you want to buy is not for sale");
            ensure!(fighter_price <= max_price, "The cat you want to buy costs more than your max price");

            <balances::Module<T> as Currency<_>>::transfer(&sender, &owner, fighter_price)?;

            Self::transfer_from(owner.clone(), sender.clone(), fighter_id)
                .expect("`owner` is shown to own the Fighter; \
                `owner` must have greater than 0 Fighters, so transfer cannot cause underflow; \
                `all_Fighter_count` shares the same type as `owned_fighter_count` \
                and minting ensure there won't ever be more than `max()` Fighters, \
                which means transfer cannot cause an overflow; \
                qed");

            fighter.price = <T::Balance as As<u64>>::sa(0);
            <CurrentFightersStorage<T>>::insert(fighter_id, fighter);

            Self::deposit_event(RawEvent::Bought(sender, owner, fighter_id, fighter_price));

            Ok(())
        }

        fn fight(origin, fighter_id_1: T::Hash, fighter_id_2: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(<CurrentFightersStorage<T>>::exists(fighter_id_1), "Fighter must exist");
            ensure!(<CurrentFightersStorage<T>>::exists(fighter_id_2), "Fighter must exist");

            let nonce = <Nonce<T>>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            let random_int: u64 = random_hash.as_ref()[8].into();

            let fighter_1 = Self::fighter(fighter_id_1);
            let fighter_2 = Self::fighter(fighter_id_2);

            let fighter_power_1 = fighter_2.strength * random_int % fighter_1.strength;
            let fighter_power_2 = fighter_1.strength * random_int % fighter_2.strength;

            let winner = if fighter_power_1 > fighter_power_2 {
                fighter_1
            } else {
                fighter_2
            };

            <CurrentFightersStorage<T>>::mutate(winner.id, |f| f.wins +=1);

            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn mint(to: T::AccountId, fighter_id: T::Hash, new_fighter: CurrentFighterVersion<T::Hash, T::Balance>) -> Result {
        ensure!(!<FighterOwner<T>>::exists(fighter_id), "Fighter already exists");

        let owned_fighter_count = Self::owned_fighter_count(&to);

        let new_owned_fighter_count = owned_fighter_count.checked_add(1)
            .ok_or("Overflow adding a new Fighter to account balance")?;

        let all_fighters_count = Self::all_fighters_count();

        let new_all_fighters_count = all_fighters_count.checked_add(1)
            .ok_or("Overflow adding a new Fighter to total supply")?;

        <CurrentFightersStorage<T>>::insert(fighter_id, new_fighter);
        <FighterOwner<T>>::insert(fighter_id, &to);

        <AllFightersArray<T>>::insert(all_fighters_count, fighter_id);
        <AllFightersCount<T>>::put(new_all_fighters_count);
        <AllFightersIndex<T>>::insert(fighter_id, all_fighters_count);

        <OwnedFightersArray<T>>::insert((to.clone(), owned_fighter_count), fighter_id);
        <OwnedFightersCount<T>>::insert(&to, new_owned_fighter_count);
        <OwnedFightersIndex<T>>::insert(fighter_id, owned_fighter_count);

        Self::deposit_event(RawEvent::Created(to, fighter_id));

        Ok(())
    }

    fn transfer_from(from: T::AccountId, to: T::AccountId, fighter_id: T::Hash) -> Result {
        let owner = Self::owner_of(fighter_id).ok_or("No owner for this Fighter")?;

        ensure!(owner == from, "'from' account does not own this Fighter");

        let owned_fighter_count_from = Self::owned_fighter_count(&from);
        let owned_fighter_count_to = Self::owned_fighter_count(&to);

        let new_owned_fighter_count_to = owned_fighter_count_to.checked_add(1)
            .ok_or("Transfer causes overflow of 'to' Fighter balance")?;

        let new_owned_fighter_count_from = owned_fighter_count_from.checked_sub(1)
            .ok_or("Transfer causes underflow of 'from' Fighter balance")?;

        // "Swap and pop"
        let fighter_index = <OwnedFightersIndex<T>>::get(fighter_id);
        if fighter_index != new_owned_fighter_count_from {
            let last_fighter_id = <OwnedFightersArray<T>>::get((from.clone(), new_owned_fighter_count_from));
            <OwnedFightersArray<T>>::insert((from.clone(), fighter_index), last_fighter_id);
            <OwnedFightersIndex<T>>::insert(last_fighter_id, fighter_index);
        }

        <FighterOwner<T>>::insert(&fighter_id, &to);
        <OwnedFightersIndex<T>>::insert(fighter_id, owned_fighter_count_to);

        <OwnedFightersArray<T>>::remove((from.clone(), new_owned_fighter_count_from));
        <OwnedFightersArray<T>>::insert((to.clone(), owned_fighter_count_to), fighter_id);

        <OwnedFightersCount<T>>::insert(&from, new_owned_fighter_count_from);
        <OwnedFightersCount<T>>::insert(&to, new_owned_fighter_count_to);
        
        Self::deposit_event(RawEvent::Transferred(from, to, fighter_id));
        
        Ok(())
    }
}