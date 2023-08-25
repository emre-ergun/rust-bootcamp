use bank1::SavingAccounts;
mod utils;

#[test]
fn should_have_a_starting_balance_0() {
    utils::common_setup();
    let account = SavingAccounts::new();
    assert_eq!(account.get_balance(), 0);
}