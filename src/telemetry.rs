#[derive(Debug)]
pub struct Telemetry {
    pub second: u32,
    pub battery_pct: u8,
    pub temp_c: f32,
    pub signal_pct: u8,
}

impl Telemetry {
    pub fn from_csv_row(row: &str) -> Result<Self, &'static str> {
        let mut parts = row.split(',');

        let second: u32 = parts
            .next()
            .ok_or("Missing second")?
            .trim()
            .parse()
            .map_err(|_| "Invalid second")?;

        let battery_pct: u8 = parts
            .next()
            .ok_or("Missing battery")?
            .trim()
            .parse()
            .map_err(|_| "Invalid battery")?;

        let temp_c: f32 = parts
            .next()
            .ok_or("Missing temperature")?
            .parse()
            .map_err(|_| "Invalid temperature")?;

        let signal_pct: u8 = parts
            .next()
            .ok_or("Missing signal")?
            .parse()
            .map_err(|_| "Invalid signal")?;

        if parts.next().is_some() {
            return Err("Too many columns");
        }

        if battery_pct > 100 || signal_pct > 100 {
            return Err("Percentage must be between 0 and 100");
        }

        if !temp_c.is_finite() {
            return Err("Temperature must be finite");
        }

        if second > 86_400 {
            return Err("Flight duration exceeds 24 hours");
        }

        Ok(Self {
            second,
            battery_pct,
            temp_c,
            signal_pct,
        })
    }
}
