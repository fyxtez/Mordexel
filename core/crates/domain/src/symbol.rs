use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub base: Asset,
    pub quote: Asset,
}

impl Symbol {
    pub fn new(base: Asset, quote: Asset) -> Self {
        Self { base, quote }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.base, self.quote)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Asset {
    BTC,
    ETH,
    SOL,
    XRP,
    BNB,
    TRX,
    ADA,
    USDT,
    USDC,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Asset::BTC => "BTC",
            Asset::ETH => "ETH",
            Asset::SOL => "SOL",
            Asset::XRP => "XRP",
            Asset::BNB => "BNB",
            Asset::TRX => "TRX",
            Asset::ADA => "ADA",
            Asset::USDT => "USDT",
            Asset::USDC => "USDC",
        };
        write!(f, "{s}")
    }
}
