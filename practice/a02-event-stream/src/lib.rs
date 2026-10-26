#[derive(Debug)]
pub enum AccountEvent {
    Variant1,
    Variant2,
}

#[derive(Debug)]
pub enum AccountSummary {
    Variant1,
    Variant2,
}

pub fn account_reduce() -> Option<AccountSummary> {
    AccountSummary {}
}
