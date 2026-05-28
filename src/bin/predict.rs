use std::io::{self, Write};
use ft_linear_regression::{estimate_price, load_thetas};


fn main()
{
    let (theta0, theta1) = load_thetas("thetas.txt");

    print!("Enter a mileage: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mileage: f64 = match input.trim().parse()
    {
        Ok(value) if value >= 0.0 => value,
        Ok(_) =>
        {
            eprintln!("Error: mileage cannot be negative");
            return;
        }
        Err(_) =>
        {
            eprintln!("Error: please enter a valid number");
            return;
        }
    };

    let price = estimate_price(mileage, theta0, theta1);
    println!("Estimated price: {:.2}", price);
}