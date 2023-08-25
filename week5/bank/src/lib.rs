pub struct SavingAccounts {
    balance: i32,
}

impl SavingAccounts {
    pub fn new() -> SavingAccounts {
        SavingAccounts { 
            balance : 0,
        }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, deposit: i32) {
        if deposit < 0 {
            panic!("Deposit value can not be negative")
        }
        self.balance += deposit;
    }

    pub fn transfer(&mut self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }
}   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_0() {
        let account = SavingAccounts::new();

        assert_eq!(0, account.balance);
    }

    #[test]
    fn should_deposit_100_into_account() {
        let mut account = SavingAccounts::new();

        account.deposit(100);

        assert_eq!(100, account.balance, "Balance should be 100");
        assert_ne!(account.balance, 0);
        assert!(account.balance != 0);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingAccounts::new();
        account.deposit(-1);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String>{
        let mut account = SavingAccounts::new();

        account.deposit(100);
        account.transfer(123456, 100)?;

        Ok(())
    }
}