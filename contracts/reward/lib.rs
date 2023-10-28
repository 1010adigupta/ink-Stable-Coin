#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
pub mod savingsRate {

    use stable_coin
    
    #[ink(storage)]
    #[derive(Storage)]   
    struct SavingsRateContract {
    time_period_to_get_interest: i32,
    // Savings rate Interest
    savings_rate_interest: i32,
    // Savings rate balance
    savings_rate_balance: Balance,
    // User savings rate deposits , i32 for timestamp at the time of depositing tokens
    user_savings_rate_deposits: Mapping<AccountId, (i32,Balance)>,
    }
    
    #[ink(event)]
    pub struct SavingsRateDeposited {
        #[ink(topic)]
        to: Option<AccountId>
        #[ink(topic)]
        amount: Balance
    }
    
    #[ink(topic)]
    pub struct SavingsRateWithdrawn{
        #[ink(topic)]
        to: Option<AccountId>
        #[ink(topic)]
        amount: Balance
    }

    pub enum Error{
        NoSavingsRateDeposit
    }

    impl SavingsRateContract {
        
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self{
                time_period_to_get_interest: ,// some constant 
                // interest rate on deposited iUSD is 5%
                savings_rate_interest:5,
                savings_rate_balance: Default::default,
                user_savings_rate_deposits:Default::default()
            }
        }

        
        #[ink(message, payable)]
        pub fn deposit_savings_rate(&mut self,amount:Balance) {
            // amount is the amount of the iUSD to deposit
            let caller = self.env().caller();
             
            // Ensure that the user has iUSD to deposit
            assert!(self.stable_coin_balance_of(caller) >= amount, "Insufficient iUSD balance");
            
            // update user's stable coin balance in the stable-coin smart contract

            let current_timestamp = self.env().block_timestamp()
            // Update user's savings rate deposit
            self.user_savings_rate_deposits.insert(&caller, (&amount,&current_timestamp));
    
            // Update the total savings rate balance
            self.savings_rate_balance += amount;
    
            // Emit an event to track the deposit
            self.env().emit_event(SavingsRateDeposited {
                from: Some(caller),
                amount,
            });
        }
     
        #[ink(message)]
        fn withdraw_savings_rate(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
    
            // Get the user's savings rate deposit
            let user_deposit = self.user_savings_rate_deposits.get(&caller).unwrap_or_default();
    
            // Ensure the user has a savings rate deposit
            if user_deposit == 0 {
                return Err(Error::NoSavingsRateDeposit);
            }
    
            let current_timestamp = self.env().block_timestamp();
            let deposited_timestamp = self.user_savings_rate_deposits.get(&caller).0;
            let time_period_to_get_interest = self.time_period_to_get_interest()
            
            
            let annual_interest_rate = self.savings_rate_interest() 
            let interest_earned = (user_deposit * annual_interest_rate) / 100;
    
            // Update the user's iUSD balance with both the deposit and interest
            if(current_timestamp - deposited_timestamp > time_period_to_get_interest){
                // will give user the interest
            } else{
                // will not give the interest
            }
    
            // Update the user's savings rate deposit to zero
            self.user_savings_rate_deposits.insert(&caller, (&0,&0));
    
            // Update the total savings rate balance
            self.savings_rate_balance -= user_deposit;
    
            self.env().emit_event(SavingsRateWithdrawn {
                to: Some(caller),
                amount: user_deposit + interest_earned,
            });
    
            Ok(())
        }


        
    }

   