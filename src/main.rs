mod telemetry;

use std::{fs, io};

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

fn main() -> io::Result<()> {
    println!("Flight Recorder started");

    let samples = load_telemetry("data/flight.csv")?;

    println!("Loaded {} telemetry samples\n", samples.len());

    for sample in &samples {
        println!(
            "[{}s] Battery: {}%, Temp: {:.1}°C, Signal: {}%",
            sample.second, sample.battery_pct, sample.temp_c, sample.signal_pct,
        );
    }

    Ok(())
}
