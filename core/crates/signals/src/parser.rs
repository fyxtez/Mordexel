use domain::{
    side::Side,
    symbol::{Asset, Symbol},
    timeframe::Timeframe,
};
use regex::Regex;
use std::sync::OnceLock;

use crate::sanitize::remove_emojis;

#[derive(Debug, Clone)]
pub struct TradingSignal {
    pub symbol: Symbol,
    pub side: Side,
    pub entry: f64,
    pub targets: Vec<f64>,
    pub timeframe: Timeframe,
    pub stop_loss: f64,
}

struct SignalRegexes {
    symbol: Regex,
    direction: Regex,
    entry: Regex,
    targets: Regex,
    stop_loss: Regex,
    disclaimer: Regex,
    timeframe: Regex,
}

fn regexes() -> &'static SignalRegexes {
    static REGEXES: OnceLock<SignalRegexes> = OnceLock::new();

    REGEXES.get_or_init(|| SignalRegexes {
        symbol: Regex::new(r"\b([A-Z]+)(USDT|USDC)\b").expect("Invalid regex: symbol"),
        timeframe: Regex::new(r"·\s*(\d+[hmdw])").expect("Invalid regex: timeframe"),
        direction: Regex::new(r"(LONG|SHORT)").expect("Invalid regex: direction"),
        entry: Regex::new(r"Entry:\s*([0-9]+\.?[0-9]*)").expect("Invalid regex: entry"),
        targets: Regex::new(r"TP[0-9]+:\s*([0-9]+\.?[0-9]*)").expect("Invalid regex: targets"),
        stop_loss: Regex::new(r"SL:\s*([0-9]+\.?[0-9]*)").expect("Invalid regex: stop_loss"),
        disclaimer: Regex::new(r"(?i)disclaimer:.*").expect("Invalid regex: disclaimer"),
    })
}

pub fn format_signal(signal: &TradingSignal) -> String {
    let direction = match signal.side {
        Side::Long => "LONG",
        Side::Short => "SHORT",
    };

    let Some(_) = signal.targets.last() else {
        return String::from("Invalid signal: missing targets");
    };

    let entry = signal.entry;
    let target = &signal.targets;
    let stop_loss = signal.stop_loss;

    format!(
        "<b>{} {}</b>\n\
        <b>Timeframe:</b> {}\n\
        <b>Entry:</b> {:.5}\n\
        <b>Take Profit 1:</b> {:.5}\n\
        <b>Take Profit 2:</b> {:.5}\n\
        <b>Take Profit 3:</b> {:.5}\n\
        <b>Stop:</b> {:.5}\n",
        signal.symbol,
        direction,
        signal.timeframe,
        entry,
        target.first().copied().unwrap_or_default(),
        target.get(1).copied().unwrap_or_default(),
        target.get(2).copied().unwrap_or_default(),
        stop_loss,
    )
}

fn parse_asset(input: &str) -> Option<Asset> {
    match input {
        "BTC" => Some(Asset::BTC),
        "ETH" => Some(Asset::ETH),
        "SOL" => Some(Asset::SOL),
        "XRP" => Some(Asset::XRP),
        "BNB" => Some(Asset::BNB),
        "TRX" => Some(Asset::TRX),
        "ADA" => Some(Asset::ADA),
        "USDT" => Some(Asset::USDT),
        "USDC" => Some(Asset::USDC),
        _ => None,
    }
}

pub fn parse_trading_signal(text: &str) -> Option<TradingSignal> {
    let re = regexes();

    let no_emoji = remove_emojis(text);
    let cleaned_text = re.disclaimer.replace_all(&no_emoji, "");

    let symbol_caps = re.symbol.captures(&cleaned_text)?;
    let base_str = symbol_caps.get(1)?.as_str();
    let quote_str = symbol_caps.get(2)?.as_str();

    let base = parse_asset(base_str)?;
    let quote = parse_asset(quote_str)?;

    let symbol = Symbol { base, quote };

    let direction_str = re.direction.captures(&cleaned_text)?.get(1)?.as_str();

    let side = match direction_str {
        "LONG" => Side::Long,
        "SHORT" => Side::Short,
        _ => return None,
    };

    let timeframe = re
        .timeframe
        .captures(&cleaned_text)
        .and_then(|c| c.get(1))
        .and_then(|m| Timeframe::parse(m.as_str()))
        .unwrap_or(Timeframe::H4);

    let entry = re
        .entry
        .captures(&cleaned_text)?
        .get(1)?
        .as_str()
        .parse::<f64>()
        .ok()?;

    let targets: Vec<f64> = re
        .targets
        .captures_iter(&cleaned_text)
        .filter_map(|cap| cap.get(1)?.as_str().parse::<f64>().ok())
        .collect();

    if targets.is_empty() {
        return None;
    }

    let stop_loss = re
        .stop_loss
        .captures(&cleaned_text)?
        .get(1)?
        .as_str()
        .parse::<f64>()
        .ok()?;

    Some(TradingSignal {
        symbol,
        side,
        entry,
        targets,
        timeframe,
        stop_loss,
    })
}
