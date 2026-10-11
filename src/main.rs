mod analyzer;
mod telemetry;

use std::{fs, io};

use analyzer::{Alert, Report, analyze};
use telemetry::Telemetry;

fn load_telemetry(path: &str) -> io::Result<Vec<Telemetry>> {
    let content = fs::read_to_string(path)?;
    let mut samples = Vec::new();

    let mut rows = content.lines();

    if rows.next() != Some("second,battery_pct,temp_c,signal_pct") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid CSV header",
        ));
    }

    for (index, row) in rows.enumerate() {
        if row.trim().is_empty() {
            continue;
        }

        let sample = Telemetry::from_csv_row(row).map_err(|message| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Line {}: {}", index + 2, message),
            )
        })?;

        samples.push(sample);
    }

    Ok(samples)
}

fn print_report(report: &Report) {
    println!("\n--- Flight Report ---");

    for alert in &report.alerts {
        match alert {
            Alert::LowBattery { second, percentage } => {
                println!("CRIT [{}s] Battery low: {}%", second, percentage);
            }

            Alert::HighTemperature { second, celsius } => {
                println!("CRIT [{}s] Temperature: {:.1}°C", second, celsius);
            }

            Alert::WeakSignal { second, percentage } => {
                println!("WARN [{}s] Signal degraded: {}%", second, percentage);
            }
        }
    }

    println!("\n--- Summary ---");
    println!("Duration: {} seconds", report.duration_seconds);
    println!("Min battery: {}%", report.min_battery_pct);
    println!("Max temperature: {:.1}°C", report.max_temp_c);
    println!("Min signal: {}%", report.min_signal_pct);
    println!("Total alerts: {}", report.alerts.len());
}

fn main() -> io::Result<()> {
    println!("Flight Recorder started");

    let samples = load_telemetry("data/flight.csv")?;

    println!("Loaded {} telemetry samples\n", samples.len());

    let report = analyze(&samples)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No telemetry samples found"))?;

    print_report(&report);

    Ok(())
}
