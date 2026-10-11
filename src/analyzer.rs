use crate::telemetry::Telemetry;

#[derive(Debug)]
pub enum Alert {
    LowBattery { second: u32, percentage: u8 },
    HighTemperature { second: u32, celsius: f32 },
    WeakSignal { second: u32, percentage: u8 },
}

#[derive(Debug)]
pub struct Report {
    pub duration_seconds: u32,
    pub min_battery_pct: u8,
    pub max_temp_c: f32,
    pub min_signal_pct: u8,
    pub alerts: Vec<Alert>,
}

pub fn analyze(samples: &[Telemetry]) -> Option<Report> {
    let first = samples.first()?;

    let mut min_second = first.second;
    let mut max_second = first.second;

    let mut report = Report {
        duration_seconds: 0,
        min_battery_pct: first.battery_pct,
        max_temp_c: first.temp_c,
        min_signal_pct: first.signal_pct,
        alerts: Vec::new(),
    };

    for sample in samples {
        min_second = min_second.min(sample.second);
        max_second = max_second.max(sample.second);

        report.min_battery_pct = report.min_battery_pct.min(sample.battery_pct);

        report.max_temp_c = report.max_temp_c.max(sample.temp_c);

        report.min_signal_pct = report.min_signal_pct.min(sample.signal_pct);

        if sample.battery_pct <= 20 {
            report.alerts.push(Alert::LowBattery {
                second: sample.second,
                percentage: sample.battery_pct,
            });
        }

        if sample.temp_c >= 70.0 {
            report.alerts.push(Alert::HighTemperature {
                second: sample.second,
                celsius: sample.temp_c,
            });
        }

        if sample.signal_pct < 40 {
            report.alerts.push(Alert::WeakSignal {
                second: sample.second,
                percentage: sample.signal_pct,
            });
        }
    }

    report.duration_seconds = max_second - min_second;

    Some(report)
}
