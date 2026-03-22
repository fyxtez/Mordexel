#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub balance: f64,
    pub available_balance: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolFilters {
    pub step_size: f64,
    pub min_qty: f64,
    pub min_notional: f64,
    pub tick_size: f64,
}