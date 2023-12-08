pub struct SavingAccounts {
    balance: i32,
}

impl SavingAccounts {
    pub fn new() -> SavingAccounts {
        SavingAccounts {
            balance: 0,
        }
    }

    pub fn get_balance(&self) -> i32 { self.balance }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("{amount} has to be bigger than zero");
        }
        self.balance += amount;
    }

    pub fn transfer(&self, acc_number: i32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred £{amount} to {acc_number}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_starting_balance_zero() {
        let account = SavingAccounts::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingAccounts::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingAccounts::new();
        account.deposit(-1);
    }

    #[test]
    fn should_tranfer_money() -> Result<(), String> {
        let mut account = SavingAccounts::new();
        account.deposit(100);
        account.transfer(123456, 100)?;
        Ok(())
    }
}