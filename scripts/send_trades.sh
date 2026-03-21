#!/usr/bin/env bash

URL="http://localhost:8656/trade"

send_trade() {
  local msg="$1"

  curl -s -X POST "$URL" \
    -H "Content-Type: application/json" \
    -d "$(jq -n --arg text "$msg" '{text: $text}')"

  echo -e "\n---"
}

# ===================== MESSAGES =====================

send_trade "🔔 ADAUSDT · 1h · 🔴 SHORT
🎯 Entry: 0.2712
🎯 Targets:
   🥇 TP1: 0.2656  ⬇️ 2.06%  ⚖️ R:R 0.50
   🥈 TP2: 0.2600  ⬇️ 4.13%
   🥉 TP3: 0.2543  ⬇️ 6.23%
🛡️ SL: 0.2824  ❗️Risk 4.13%"

send_trade "🔔 BNBUSDT · 1h · 🔴 SHORT
🎯 Entry: 648.13
🎯 Targets:
   🥇 TP1: 640.81  ⬇️ 1.13%  ⚖️ R:R 0.50
   🥈 TP2: 633.49  ⬇️ 2.26%
   🥉 TP3: 626.17  ⬇️ 3.39%
🛡️ SL: 662.77  ❗️Risk 2.26%"

send_trade "🔔 XRPUSDT · 30m · 🔴 SHORT
🎯 Entry: 1.4640
🎯 Targets:
   🥇 TP1: 1.4393  ⬇️ 1.69%  ⚖️ R:R 0.50
   🥈 TP2: 1.4146  ⬇️ 3.37%
   🥉 TP3: 1.3899  ⬇️ 5.06%
🛡️ SL: 1.5134  ❗️Risk 3.37%"

send_trade "🔔 ETHUSDT · 30m · 🔴 SHORT
🎯 Entry: 2269.34
🎯 Targets:
   🥇 TP1: 2248.63  ⬇️ 0.91%  ⚖️ R:R 0.50
   🥈 TP2: 2227.92  ⬇️ 1.83%
   🥉 TP3: 2207.21  ⬇️ 2.74%
🛡️ SL: 2310.76  ❗️Risk 1.83%"

send_trade "🔔 SOLUSDT · 30m · 🔴 SHORT
🎯 Entry: 92.00
🎯 Targets:
   🥇 TP1: 91.07  ⬇️ 1.01%  ⚖️ R:R 0.50
   🥈 TP2: 90.15  ⬇️ 2.01%
   🥉 TP3: 89.22  ⬇️ 3.02%
🛡️ SL: 93.85  ❗️Risk 2.01%"

echo "Done."