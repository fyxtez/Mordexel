use domain::timeframe::Timeframe;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpStrategy {
    /// Use only TP2 as the active TP target.
    /// Intended for custom partial-take-profit logic later.
    Tp2Adjusted,

    /// Safe TP2/TP3 split:
    /// - [tp1, tp2, tp3, ..] => [tp2, tp3]
    /// - [tp1, tp2]          => [tp1, tp2]
    /// - [tp1]               => [tp1]
    Tp2Tp3Split,

    /// Return targets exactly as received.
    Copied,
}

pub fn resolve_tp_targets(
    strategy: TpStrategy,
    targets: &[f64],
    _timeframe: Timeframe,
) -> Vec<f64> {
    match strategy {
        TpStrategy::Tp2Adjusted => match targets {
            [_, tp2, ..] => vec![*tp2],
            [tp1] => vec![*tp1],
            [] => vec![],
        },

        TpStrategy::Tp2Tp3Split => match targets {
            [_, tp2, tp3, ..] => vec![*tp2, *tp3],
            [tp1, tp2] => vec![*tp1, *tp2],
            [tp1] => vec![*tp1],
            [] => vec![],
        },

        TpStrategy::Copied => targets.to_vec(),
    }
}