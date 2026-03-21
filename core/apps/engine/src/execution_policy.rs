use std::collections::{HashMap, HashSet};

use domain::{
    symbol::{Asset, Symbol},
    timeframe::Timeframe,
};

#[derive(Debug, Clone)]
pub struct ExecutionPolicy {
    allowed: HashMap<Timeframe, HashSet<Symbol>>,
}

impl ExecutionPolicy {
    pub fn strict_profit_only() -> Self {
        use Asset::*;
        let mut allowed = HashMap::new();

        allowed.insert(
            Timeframe::M30,
            HashSet::from([
                Symbol::new(ADA, USDT),
                Symbol::new(SOL, USDT),
                Symbol::new(XRP, USDT),
                Symbol::new(TRX, USDT),
            ]),
        );

        allowed.insert(
            Timeframe::H1,
            HashSet::from([Symbol::new(SOL, USDT), Symbol::new(BTC, USDT)]),
        );

        Self { allowed }
    }

    pub fn is_allowed(&self, timeframe: Timeframe, symbol: &Symbol) -> bool {
        self.allowed
            .get(&timeframe)
            .map(|symbols| symbols.contains(symbol))
            .unwrap_or(false)
    }
}

// === No Cohort Filter (Full Dataset) ===

// === Cohort Filter ===
// (last None days)
// Trades in cohort: 295

// === Dataset Info ===
// Start Date: 2026-01-21 12:32:18 UTC
// End Date  : 2026-03-18 17:05:53 UTC
// Duration  : 56 days

// === Timeframe Statistics ===

// Timeframe: 30m
// Total: 153
// TP1: 115 (75.16%)
// TP2: 95 (62.09%)
// TP3: 78 (50.98%)
// SL : 34 (22.22%)

// Timeframe: 1h
// Total: 82
// TP1: 59 (71.95%)
// TP2: 42 (51.22%)
// TP3: 34 (41.46%)
// SL : 20 (24.39%)

// Timeframe: 2h
// Total: 45
// TP1: 30 (66.67%)
// TP2: 21 (46.67%)
// TP3: 14 (31.11%)
// SL : 13 (28.89%)

// Timeframe: 4h
// Total: 15
// TP1: 6 (40.00%)
// TP2: 6 (40.00%)
// TP3: 4 (26.67%)
// SL : 9 (60.00%)

// === True First Outcome Expectancy (TP1 Model) ===

// ----------------------------
// Timeframe: 30m
// Total Trades: 153
// Resolved Trades: 148
// Unresolved: 5 (3.27%)
// Hard SL (before TP1): 33 (21.57%)
// Total R: 24.500
// Expectancy per trade: 0.166R
// Max win streak  : 21
// Max loss streak : 7
// Avg win streak  : 6.76
// Avg loss streak : 2.06

// ----------------------------
// Timeframe: 1h
// Total Trades: 82
// Resolved Trades: 78
// Unresolved: 4 (4.88%)
// Hard SL (before TP1): 19 (23.17%)
// Total R: 10.500
// Expectancy per trade: 0.135R
// Max win streak  : 13
// Max loss streak : 3
// Avg win streak  : 3.93
// Avg loss streak : 1.36

// ----------------------------
// Timeframe: 2h
// Total Trades: 45
// Resolved Trades: 43
// Unresolved: 2 (4.44%)
// Hard SL (before TP1): 13 (28.89%)
// Total R: 2.000
// Expectancy per trade: 0.047R
// Max win streak  : 8
// Max loss streak : 3
// Avg win streak  : 3.75
// Avg loss streak : 1.86

// ----------------------------
// Timeframe: 4h
// Total Trades: 15
// Resolved Trades: 15
// Unresolved: 0 (0.00%)
// Hard SL (before TP1): 9 (60.00%)
// Total R: -6.000
// Expectancy per trade: -0.400R
// Max win streak  : 4
// Max loss streak : 4
// Avg win streak  : 2.00
// Avg loss streak : 3.00

// === Per-Timeframe Symbol Winrate (TP1-first model) ===

// --- Timeframe: 30m (R Volatility) ---
// Resolved N       : 148
// Mean R (μ)       : 0.1655
// StdDev R (σ)     : 0.6265
// Stability (μ/σ)  : 0.2642
// Final Equity (R) : 24.500
// Peak Equity (R)  : 24.500
// Max Drawdown (R) : 7.000

// --- Timeframe: 30m ---
// HHI (trade concentration): 0.1449
// Symbol   Total  TP1%   TP2%   TP3%   SL%    WR%    Exp(R)  P(TP2|TP1)  P(TP3|TP2)  SL_after_TP1%  P(SL|TP1)%  TradeShare%  RShareSigned%  RShareAbs%  LB95%  Gap95(pp)  SAFE  N_R  Mu      Sigma   Mu/Sigma
// SOLUSDT  20     80.00  65.00  55.00  15.00  85.00  0.250   81.25       84.62       0.00           0.00        13.07        20.41          20.41       63.96  -2.71      N     19   0.2632  0.5470  0.4811
// ADAUSDT  19     84.21  78.95  52.63  15.79  84.21  0.263   93.75       66.67       0.00           0.00        12.42        20.41          20.41       62.43  -4.23      N     19   0.2632  0.5470  0.4811
// XRPUSDT  22     77.27  72.73  54.55  18.18  81.82  0.205   94.12       75.00       0.00           0.00        14.38        18.37          18.37       61.48  -5.18      N     21   0.2143  0.5890  0.3638
// TRXUSDT  27     77.78  62.96  55.56  25.93  77.78  0.167   80.95       88.24       3.70           4.76        17.65        18.37          18.37       59.24  -7.42      N     27   0.1667  0.6236  0.2673
// BNBUSDT  21     71.43  57.14  47.62  23.81  76.19  0.119   80.00       83.33       0.00           0.00        13.73        10.20          10.20       54.91  -11.76     N     20   0.1250  0.6495  0.1925
// ETHUSDT  24     70.83  50.00  41.67  25.00  75.00  0.104   70.59       83.33       0.00           0.00        15.69        10.20          10.20       55.10  -11.57     N     23   0.1087  0.6587  0.1650
// BTCUSDT  20     65.00  50.00  50.00  30.00  70.00  0.025   76.92       100.00      0.00           0.00        13.07        2.04           2.04        48.10  -18.56     N     19   0.0263  0.6972  0.0377

// --- Timeframe: 1h (R Volatility) ---
// Resolved N       : 78
// Mean R (μ)       : 0.1346
// StdDev R (σ)     : 0.6480
// Stability (μ/σ)  : 0.2077
// Final Equity (R) : 10.500
// Peak Equity (R)  : 10.500
// Max Drawdown (R) : 4.500

// --- Timeframe: 1h ---
// HHI (trade concentration): 0.1532
// Symbol   Total  TP1%    TP2%   TP3%   SL%    WR%     Exp(R)  P(TP2|TP1)  P(TP3|TP2)  SL_after_TP1%  P(SL|TP1)%  TradeShare%  RShareSigned%  RShareAbs%  LB95%  Gap95(pp)  SAFE  N_R  Mu       Sigma   Mu/Sigma
// SOLUSDT  9      100.00  77.78  66.67  0.00   100.00  0.500   77.78       85.71       0.00           0.00        10.98        42.86          29.03       70.08  +3.42      Y     9    0.5000   0.0000  0.0000
// BTCUSDT  11     81.82   54.55  45.45  9.09   90.91   0.318   66.67       83.33       0.00           0.00        13.41        33.33          22.58       62.26  -4.40      N     10   0.3500   0.4500  0.7778
// ETHUSDT  7      71.43   42.86  42.86  14.29  85.71   0.214   60.00       100.00      0.00           0.00        8.54         14.29          9.68        48.69  -17.98     N     6    0.2500   0.5590  0.4472
// BNBUSDT  12     66.67   58.33  50.00  25.00  75.00   0.083   87.50       85.71       0.00           0.00        14.63        9.52           6.45        46.77  -19.90     N     11   0.0909   0.6680  0.1361
// TRXUSDT  15     73.33   60.00  40.00  33.33  73.33   0.100   81.82       66.67       6.67           9.09        18.29        14.29          9.68        48.05  -18.62     N     15   0.1000   0.6633  0.1508
// XRPUSDT  11     72.73   45.45  36.36  27.27  72.73   0.091   62.50       80.00       0.00           0.00        13.41        9.52           6.45        43.43  -23.23     N     11   0.0909   0.6680  0.1361
// ADAUSDT  17     52.94   29.41  23.53  41.18  58.82   -0.147  55.56       80.00       0.00           0.00        20.73        -23.81         16.13       36.01  -30.66     N     16   -0.1562  0.7441  -0.2100

// --- Timeframe: 2h (R Volatility) ---
// Resolved N       : 43
// Mean R (μ)       : 0.0465
// StdDev R (σ)     : 0.6971
// Stability (μ/σ)  : 0.0667
// Final Equity (R) : 2.000
// Peak Equity (R)  : 2.000
// Max Drawdown (R) : 3.500

// --- Timeframe: 2h ---
// HHI (trade concentration): 0.1516
// Symbol   Total  TP1%   TP2%   TP3%   SL%    WR%    Exp(R)  P(TP2|TP1)  P(TP3|TP2)  SL_after_TP1%  P(SL|TP1)%  TradeShare%  RShareSigned%  RShareAbs%  LB95%  Gap95(pp)  SAFE  N_R  Mu       Sigma   Mu/Sigma
// SOLUSDT  6      83.33  83.33  50.00  16.67  83.33  0.250   100.00      60.00       0.00           0.00        13.33        75.00          25.00       43.65  -23.02     N     6    0.2500   0.5590  0.4472
// XRPUSDT  6      66.67  33.33  16.67  16.67  83.33  0.167   50.00       50.00       0.00           0.00        13.33        50.00          16.67       43.65  -23.02     N     5    0.2000   0.6000  0.3333
// BTCUSDT  5      80.00  40.00  20.00  20.00  80.00  0.200   50.00       50.00       0.00           0.00        11.11        50.00          16.67       37.55  -29.11     N     5    0.2000   0.6000  0.3333
// ADAUSDT  4      75.00  75.00  75.00  25.00  75.00  0.125   100.00      100.00      0.00           0.00        8.89         25.00          8.33        30.06  -36.60     N     4    0.1250   0.6495  0.1925
// TRXUSDT  9      66.67  44.44  33.33  33.33  66.67  0.000   66.67       75.00       0.00           0.00        20.00        0.00           0.00        35.42  -31.25     N     9    0.0000   0.7071  0.0000
// BNBUSDT  8      50.00  12.50  12.50  37.50  62.50  -0.125  25.00       100.00      0.00           0.00        17.78        -50.00         16.67       30.57  -36.09     N     7    -0.1429  0.7423  -0.1925
// ETHUSDT  7      57.14  57.14  28.57  42.86  57.14  -0.143  100.00      50.00       0.00           0.00        15.56        -50.00         16.67       25.05  -41.62     N     7    -0.1429  0.7423  -0.1925

// --- Timeframe: 4h (R Volatility) ---
// Resolved N       : 15
// Mean R (μ)       : -0.4000
// StdDev R (σ)     : 0.7606
// Stability (μ/σ)  : -0.5259
// Final Equity (R) : -6.000
// Peak Equity (R)  : 0.500
// Max Drawdown (R) : 6.500

// --- Timeframe: 4h ---
// HHI (trade concentration): 0.2178
// Symbol   Total  TP1%    TP2%    TP3%    SL%    WR%     Exp(R)  P(TP2|TP1)  P(TP3|TP2)  SL_after_TP1%  P(SL|TP1)%  TradeShare%  RShareSigned%  RShareAbs%  LB95%  Gap95(pp)  SAFE  N_R  Mu       Sigma   Mu/Sigma
// ETHUSDT  1      100.00  100.00  100.00  0.00   100.00  0.500   100.00      100.00      0.00           0.00        6.67         -8.33          6.25        20.65  -46.01     N     1    0.5000   0.0000  0.0000
// BNBUSDT  1      100.00  100.00  0.00    0.00   100.00  0.500   100.00      0.00        0.00           0.00        6.67         -8.33          6.25        20.65  -46.01     N     1    0.5000   0.0000  0.0000
// TRXUSDT  2      50.00   50.00   50.00   50.00  50.00   -0.250  100.00      100.00      0.00           0.00        13.33        8.33           6.25        9.45   -57.21     N     2    -0.2500  0.7500  -0.3333
// SOLUSDT  3      33.33   33.33   33.33   66.67  33.33   -0.500  100.00      100.00      0.00           0.00        20.00        25.00          18.75       6.15   -60.52     N     3    -0.5000  0.7071  -0.7071
// XRPUSDT  3      33.33   33.33   33.33   66.67  33.33   -0.500  100.00      100.00      0.00           0.00        20.00        25.00          18.75       6.15   -60.52     N     3    -0.5000  0.7071  -0.7071
// ADAUSDT  5      20.00   20.00   0.00    80.00  20.00   -0.700  100.00      0.00        0.00           0.00        33.33        58.33          43.75       3.62   -63.04     N     5    -0.7000  0.6000  -1.1667
