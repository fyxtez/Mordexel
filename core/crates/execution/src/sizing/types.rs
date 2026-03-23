use crate::error::ExecutionError;

#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    pub quantity: f64,
    pub leverage: u32,
}

#[derive(Debug, Clone)]
pub struct MarginSizingConfig {
    pub margin_pct: f64,      // 0.01 = 1%
    pub leverage_safety: f64, // e.g. 0.70
    pub max_leverage: u32,    // e.g. 100
}

impl MarginSizingConfig {
    pub fn new(
        margin_pct: f64,
        leverage_safety: f64,
        max_leverage: u32,
    ) -> Result<Self, ExecutionError> {
        if margin_pct <= 0.0 {
            return Err(ExecutionError::Internal {
                message: "margin_pct must be > 0".into(),
            });
        }

        if !(0.0 < leverage_safety && leverage_safety <= 1.0) {
            return Err(ExecutionError::Internal {
                message: "leverage_safety must be in (0, 1]".into(),
            });
        }

        if max_leverage == 0 {
            return Err(ExecutionError::Internal {
                message: "max_leverage must be > 0".into(),
            });
        }

        Ok(Self {
            margin_pct,
            leverage_safety,
            max_leverage,
        })
    }
}
