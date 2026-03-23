use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Long,
    Short,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Side::Long => "BUY",
            Side::Short => "SELL",
        };
        write!(f, "{s}")
    }
}
