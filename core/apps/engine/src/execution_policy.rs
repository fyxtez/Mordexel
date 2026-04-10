use chrono::{Datelike, Timelike, Utc};
use std::collections::{HashMap, HashSet};
use tracing::{info, warn};

use domain::{
    rejected_trade::RejectionReason,
    symbol::{Asset, Symbol},
    timeframe::Timeframe,
};

#[derive(Debug, Clone)]
pub struct ExecutionPolicy {
    allowed: HashMap<Timeframe, HashSet<Symbol>>,
    blocked_sessions: HashMap<Timeframe, Vec<(u32, u32)>>, // (start_hour, end_hour) UTC
    blocked_weekdays: HashMap<Timeframe, HashSet<u32>>,    // 0=Mon..6=Sun
}

impl ExecutionPolicy {
    pub fn continuation_v1() -> Self {
        use Asset::*;
        let mut allowed = HashMap::new();
        let mut blocked_sessions = HashMap::new();
        let mut blocked_weekdays = HashMap::new();

        // ── Symbol filter ─────────────────────────────────
        allowed.insert(
            Timeframe::M30,
            HashSet::from([
                Symbol::new(ADA, USDT),
                Symbol::new(SOL, USDT),
                Symbol::new(XRP, USDT),
                Symbol::new(TRX, USDT),
            ]),
        );

        // allowed.insert(
        //     Timeframe::H1,
        //     HashSet::from([
        //         Symbol::new(SOL, USDT),
        //         Symbol::new(BTC, USDT),
        //         Symbol::new(ETH, USDT),
        //     ]),
        // );

        // ── Session blocks (UTC hours) ────────────────────
        // 30m: New York session 18-24 UTC → 0.000 avgR, 66.7% WR
        blocked_sessions.insert(Timeframe::M30, vec![(18, 24)]);

        // 1h: sessions are all decent, no blocks needed
        // blocked_sessions.insert(Timeframe::H1, vec![]);

        // ── Weekday blocks ────────────────────────────────
        // 30m: Tuesday → 60% WR, -0.100 avgR (only negative day)
        blocked_weekdays.insert(Timeframe::M30, HashSet::from([1])); // 1 = Tuesday

        // 1h: Saturday + Sunday → 63.6% / 66.7% WR
        // blocked_weekdays.insert(Timeframe::H1, HashSet::from([5, 6])); // Sat, Sun

        Self {
            allowed,
            blocked_sessions,
            blocked_weekdays,
        }
    }

    pub fn evaluate(&self, timeframe: Timeframe, symbol: &Symbol) -> Result<(), RejectionReason> {
        let symbol_ok = self
            .allowed
            .get(&timeframe)
            .map(|symbols| symbols.contains(symbol))
            .unwrap_or(false);

        if !symbol_ok {
            return Err(RejectionReason::SymbolNotAllowed);
        }

        let now = Utc::now();

        if let Some(sessions) = self.blocked_sessions.get(&timeframe) {
            let hour = now.hour();
            for &(start, end) in sessions {
                if hour >= start && hour < end {
                    return Err(RejectionReason::BlockedSession);
                }
            }
        }

        if let Some(days) = self.blocked_weekdays.get(&timeframe) {
            let dow = now.weekday().num_days_from_monday();
            if days.contains(&dow) {
                return Err(RejectionReason::BlockedWeekday);
            }
        }

        Ok(())
    }

    pub fn log_todays_plan(&self) {
        let now = Utc::now();
        let hour = now.hour();
        let dow = now.weekday().num_days_from_monday();

        let day_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        let day_name = day_names.get(dow as usize).unwrap_or(&"???");

        info!(
            day = day_name,
            hour = hour,
            "execution policy check for today"
        );

        for (timeframe, symbols) in &self.allowed {
            let mut active: Vec<String> = Vec::new();
            let mut blocked_reasons: Vec<String> = Vec::new();

            // Check weekday block for this timeframe
            let weekday_blocked = self
                .blocked_weekdays
                .get(timeframe)
                .map(|days| days.contains(&dow))
                .unwrap_or(false);

            if weekday_blocked {
                blocked_reasons.push(format!("{} is a blocked weekday", day_name));
            }

            // Check session block for this timeframe at current hour
            let session_blocked = self
                .blocked_sessions
                .get(timeframe)
                .map(|sessions| {
                    sessions
                        .iter()
                        .any(|&(start, end)| hour >= start && hour < end)
                })
                .unwrap_or(false);

            if session_blocked {
                blocked_reasons.push(format!("hour {} is in a blocked session", hour));
            }

            for symbol in symbols {
                active.push(symbol.to_string());
            }

            active.sort();

            if blocked_reasons.is_empty() {
                info!(
                    timeframe = %timeframe,
                    symbols = %active.join(", "),
                    "ACTIVE — no blocks in effect"
                );
            } else {
                warn!(
                    timeframe = %timeframe,
                    symbols = %active.join(", "),
                    blocks = %blocked_reasons.join("; "),
                    "BLOCKED — all {} signals will be rejected right now", timeframe
                );
            }
        }
    }
}
