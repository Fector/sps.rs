mod handlers {}

pub struct WalletHandler {}

impl WalletHandler {
    pub async fn get_wallets_list(&self) {}
    pub async fn get_wallet(&self) {}
    pub async fn post_wallet(&self) {}
}

pub struct TransferHandler {}

impl TransferHandler {
    pub async fn post_deposit(&self) {}
    pub async fn post_transfer(&self) {}
}

pub struct HistoryHandler {}

impl HistoryHandler {
    pub async fn get_transactions_list(&self) {}
}
