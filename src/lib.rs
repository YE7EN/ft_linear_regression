use std::error::Error;

pub struct DataPoint
{
    pub mileage: f64,
    pub price: f64,
}

pub fn load_dataset(path: &str) -> Result<Vec<DataPoint>, Box<dyn Error>>
{
    let mut reader = csv::Reader::from_path(path)?;
    let mut data = Vec::new();

    for result in reader.records()
    {
        let record = result?;
        let mileage: f64 = record[0].parse()?;
        let price: f64 = record[1].parse()?;
        data.push(DataPoint { mileage, price });
    }

    Ok(data)
}

use std::fs;

pub fn estimate_price(mileage: f64, theta0: f64, theta1: f64) -> f64
{
    theta0 + theta1 * mileage
}

pub fn load_thetas(path: &str) -> (f64, f64)
{
    match fs::read_to_string(path)
    {
        Ok(content) =>
        {
            let parts: Vec<&str> = content.trim().split(',').collect();
            if parts.len() != 2
            {
                return (0.0, 0.0);
            }
            let theta0 = parts[0].parse().unwrap_or(0.0);
            let theta1 = parts[1].parse().unwrap_or(0.0);
            (theta0, theta1)
        }
        Err(_) => (0.0, 0.0),
    }
}


pub fn train(data: &[DataPoint], learning_rate: f64, iterations: usize) -> (f64, f64) {
    let m = data.len() as f64;

    let km_min = data.iter().map(|p| p.mileage).fold(f64::INFINITY, f64::min);
    let km_max = data.iter().map(|p| p.mileage).fold(f64::NEG_INFINITY, f64::max);

    let normalized: Vec<(f64, f64)> = data
        .iter()
        .map(|p| ((p.mileage - km_min) / (km_max - km_min), p.price))
        .collect();

        let mut theta0 = 0.0;
        let mut theta1 = 0.0;

        for _ in 0..iterations {
            let mut sum0 = 0.0;
            let mut sum1 = 0.0;

            for &(km, price) in &normalized {
                let error = estimate_price(km, theta0, theta1) - price;
                sum0 += error;
                sum1 += error * km;
            }

            let tmp0 = learning_rate * (sum0 / m);
            let tmp1 = learning_rate * (sum1 / m);

            theta0 -= tmp0;
            theta1 -= tmp1;
        }
        
        let denorm_theta1 = theta1 / (km_max - km_min);
        let denorm_theta0 = theta0 - theta1 * km_min / (km_max - km_min);

        (denorm_theta0, denorm_theta1)
}

pub fn save_thetas(path: &str, theta0: f64, theta1: f64) -> std::io::Result<()> {
    fs::write(path, format!("{},{}", theta0, theta1))
}