use bank::SavingAccounts;

#[test]
fn should_have_starting_balance_zero() {
    let account = SavingAccounts::new();

    assert_eq!(account.get_balance(), 0);
}