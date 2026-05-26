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
            let parts: Vecc<&str> = content.trim().split(',').collect();
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