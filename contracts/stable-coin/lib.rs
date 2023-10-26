#![cfg_attr(not(feature = "std"), no_std, no_main)]
// pub use self::token::StableCoinContractRef;

#[openbrush::implementation(PSP22, PSP22Metadata, PSP22Mintable, PSP22Burnable, Ownable)]
#[openbrush::contract]
pub mod token {
    use ink::storage::Mapping;
    use dia_oracle_getter::OracleGetters;
    use ink::contract_ref;
    use openbrush::{ contracts::ownable::*, modifiers, traits::{ Storage, String } };

    /// Define the storage for PSP22 data, Metadata data and Ownable data
    #[ink(storage)]
    #[derive(Storage)]
    pub struct StableCoinContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        /// Metadata of Stable Coin Token
        #[storage_field]
        metadata: metadata::Data,
        /// Mapping of user address to total collateral deposited amount
        collateral_balance_of_user: Mapping<AccountId, Balance>,
        /// Oracle Instantiation
        oracle: contract_ref!(OracleGetters),
        /// Health Factor of an user
        health_factor: Mapping<AccountId, u128>,
    }

    #[ink(event)]
    pub struct CollateralDepositedAndStableCoinMinted {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct CollateralRedeemedAndStableCoinBurned {
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct UserLiquidated {
        #[ink(topic)]
        user: Option<AccountId>,
        #[ink(topic)]
        liquidated_collateral_amount: Balance,
        #[ink(topic)]
        burned_stable_coin_amount: Balance,
    }

    /// StableCoin Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Health Factor is broken
        BadHealthFactor,
        /// Health Factor is ok
        HealthFactorOk,
        /// Health Factor is not improved
        HealthFactorNotImproved,
    }

    impl StableCoinContract {
        
        /// Constructor is initialized with Stable Coin Metadata
        #[ink(constructor)]
        pub fn new(oracle_address: AccountId) -> Self {
            let caller = Self::env().caller();

            let mut instance = Self {
                psp22: Default::default(),
                ownable: Default::default(),
                metadata: Default::default(),
                collateral_balance_of_user: Default::default(),
                oracle: oracle_address.into(),
                health_factor: Default::default(),
            };

            instance.metadata.name.set(&Some(String::from("iUSD")));
            instance.metadata.symbol.set(&Some(String::from("iUSD")));
            instance.metadata.decimals.set(&18);
            ownable::Internal::_init_with_owner(&mut instance, caller);

            instance
        }

        /// pub function to deposit collateral and mint stable coin to the caller
        #[ink(message, payable)]
        pub fn deposit_collateral_and_mint_stable_coin(&mut self) {
            let caller = self.env().caller();
            let contract_address = Self::env().account_id();
            let amount = Self::env().transferred_value();
            self._deposit_collateral(contract_address, amount);
            let stable_coin_to_mint_from_collateral =
                self.get_stable_coin_amount_from_collateral(amount);
            let actual_stable_coin_to_mint = stable_coin_to_mint_from_collateral / 2;
            self.mint(caller, actual_stable_coin_to_mint);
            let health_factor = self.calculate_health_factor(caller);
            self._set_health_factor(caller, health_factor);
            self.env().emit_event(CollateralDepositedAndStableCoinMinted {
                from: Some(caller),
                amount,
            });
            self.collateral_balance_of_user.insert(&caller, &amount);
        }

        /// pub function to get amount of stable coin from collateral deposited
        #[ink(message)]
        pub fn get_stable_coin_amount_from_collateral(&self, amount: Balance) -> Balance {
            let collateral_price = self.get_price_feed(String::from("AZERO/USD"));
            let stable_coin_amount =
                (amount * collateral_price.unwrap_or_default()) / 1000000000000000000000000000000;
            stable_coin_amount
        }

        /// pub function to redeem collateral and burn stable coin from the caller
        #[ink(message, payable)]
        pub fn redeem_collateral_for_stable_coin(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            let amount = Self::env().transferred_value();
            if self.calculate_health_factor(caller) < 1 {
                return Err(Error::BadHealthFactor);
            }
            self.burn(caller, amount);
            self._redeem_collateral(caller, amount);
            if self.calculate_health_factor(caller) < 1 {
                return Err(Error::BadHealthFactor);
            } else {
                self.env().emit_event(CollateralRedeemedAndStableCoinBurned {
                    to: Some(caller),
                    amount,
                });
                return Ok(());
            }
        }

        /// pub function to calculate health factor of an user, health factor is calculated as ration of collateral amount to stable coin amount, with LIQUIDATION_THRESHOLD as 2, which allows stable coin to be 200% collateralized
        #[ink(message)]
        pub fn calculate_health_factor(&mut self, user: AccountId) -> u128 {
            let user_collateral_balance = self.collateral_balance_of(user);
            let user_stable_coin_balance = self.stable_coin_balance_of(user);
            let collateral_price = self.get_price_feed(String::from("AZERO/USD"));
            let user_collateral_amount =
                user_collateral_balance * collateral_price.unwrap_or_default();
            let LIQUIDATION_THRESHOLD = 2;
            let health_factor =
                user_collateral_amount /
                user_stable_coin_balance /
                1000000000000000000000000000000 /
                LIQUIDATION_THRESHOLD;
            health_factor
        }

        ///pub function to liquidate an user, anyone can call this function to liquidate an user, if health factor of an user is less than 1, then user can be liquidated. 10% bonus collateral is given to the liquidator as a reward from over collaterized token
        #[ink(message, payable)]
        pub fn liquidate(&mut self, user: AccountId) -> Result<(), Error> {
            let collateral_amount = Self::env().transferred_value();
            let stable_coin_to_burn =
                self.get_stable_coin_amount_from_collateral(collateral_amount);
            let caller = self.env().caller();
            let starting_user_health_factor = self.calculate_health_factor(user);
            if starting_user_health_factor > 1 {
                return Err(Error::HealthFactorOk);
            }

            let bonus_collateral = (collateral_amount * 10) / 100;

            self.burn(caller, stable_coin_to_burn);

            self._redeem_collateral(caller, collateral_amount + bonus_collateral);

            let ending_user_health_factor = self.calculate_health_factor(user);

            if ending_user_health_factor < starting_user_health_factor {
                return Err(Error::HealthFactorNotImproved);
            }

            self.env().emit_event(UserLiquidated {
                user: Some(user),
                liquidated_collateral_amount: collateral_amount,
                burned_stable_coin_amount: stable_coin_to_burn,
            });

            Ok(())
        }

        /// pub function to get user information, returns user collateral balance, stable coin balance and health factor
        #[ink(message)]
        pub fn get_user_information(&mut self) -> (Balance, Balance, u128) {
            let caller = self.env().caller();
            let user_collateral_balance = self.collateral_balance_of(caller);
            let user_stable_coin_balance = self.stable_coin_balance_of(caller);
            let user_health_factor = self.calculate_health_factor(caller);
            (user_collateral_balance, user_stable_coin_balance, user_health_factor)
        }

        /// Returns the stable coin balance for the specified `user`.
        #[ink(message)]
        pub fn stable_coin_balance_of(&mut self, user: AccountId) -> Balance {
            let balance = psp22::Internal::_balance_of(self, &user);
            balance
        }

        /// Returns the latest price of collateral token in USD by accessing DIA oracles on-chain.
        #[ink(message)]
        pub fn get_price_feed(&self, key: String) -> Option<u128> {
            let core::prelude::v1::Some((feed_time, price_feed)) =
                self.oracle.get_latest_price(key) else {
                return None;
            };
            Some(price_feed)
        }

        /// Returns the collateral balance for the specified `user`.
        #[ink(message)]
        pub fn collateral_balance_of(&self, user: AccountId) -> Balance {
            self.collateral_balance_of_user.get(&user).unwrap_or_default()
        }

        /// Returns the contract balance.
        #[ink(message)]
        pub fn contract_balance(&self) -> Balance {
            Self::env().balance()
        }

        /// Returns the contract address.
        #[ink(message)]
        pub fn get_contract_address(&self) -> AccountId {
            Self::env().account_id()
        }

        /// Internal Functions
        fn _set_health_factor(&mut self, user: AccountId, health_factor: u128) {
            self.health_factor.insert(&user, &health_factor);
        }

        pub fn mint(&mut self, to: AccountId, value: Balance) {
            psp22::Internal::_mint_to(self, to, value);
        }

        pub fn burn(&mut self, from: AccountId, value: Balance) {
            psp22::Internal::_burn_from(self, from, value);
        }

        fn _deposit_collateral(&mut self, to: AccountId, amount: Balance) {
            self.collateral_balance_of_user.insert(&to, &amount);
            Self::env().transfer(to, amount);
        }

        fn _redeem_collateral(&mut self, from: AccountId, amount: Balance) {
            let from_balance = self.collateral_balance_of(from);
            self.collateral_balance_of_user.insert(&from, &(from_balance - amount));
            Self::env().transfer(from, amount);
            let health_factor = self.calculate_health_factor(from);
            self._set_health_factor(from, health_factor);
        }
    }
}
