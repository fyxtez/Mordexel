# Mordexel Engine

Mordexel is a modular, event-driven trading engine that processes signals and executes trades on Binance USDⓈ-M Futures with built-in risk controls.

---

## What It Does

* Receives trade signals (e.g. Telegram, API)
* Converts them into structured trade intents
* Evaluates trades against execution policies
* Sizes positions based on margin + leverage rules
* Executes trades on Binance Futures
* Applies stop-loss and take-profit logic

---

## Architecture

Pipeline-based design:

Ingress → Builder → Evaluator → Executor

* **Ingress**: External signals (Telegram, etc.)
* **Builder**: Parses raw input into `TradeIntent`
* **Evaluator**: Filters trades via `ExecutionPolicy`
* **Executor**: Handles sizing + execution

---

## Core Concepts

* **ExecutionPolicy**
  Defines which trades are allowed (symbol + timeframe)

* **Sizing Engine**
  Calculates position size using:

  * margin %
  * leverage safety
  * exchange constraints (min qty, step size, notional)

* **Risk Controls**

  * Stop-loss enforcement
  * Take-profit handling
  * Leverage limits

---

## Getting Started

## Notes

* Currently supports **Binance USDⓈ-M Futures only**
* Designed for **rule-based execution**, not strategy generation
* Works best when paired with external signal providers

---

## Philosophy

Mordexel is intentionally simple:

> It does not try to be smart.
> It executes decisions with precision.

---

## Related

* Aegis (AI assistant layer for monitoring + reasoning) *(optional pairing)*

---

## Status

Early-stage, actively evolving.
