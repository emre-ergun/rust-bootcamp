/// A Savings account
pub struct SavingAccounts {
    balance: i32,
}

impl SavingAccounts {
    /// Creates a 'Savings Account' with the balance of 0
    /// 
    /// #Examples
    /// 
    /// ```
    /// use bank::SavingAccounts;
    /// let account = SavingAccounts::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    /// 
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