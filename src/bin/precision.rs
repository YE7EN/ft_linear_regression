use ft_linear_regression::{load_dataset, load_thetas, precision};

fn main() {
    let data = match load_dataset("data.csv") {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Error: could not load data.csv");
            eprintln!("Make sure data.csv is in the current directory.");
            return;
        }
    };

    let (theta0, theta1) = load_thetas("thetas.txt");

    if theta0 == 0.0 && theta1 == 0.0 {
        println!("Warning: model not trained yet (thetas are 0). Run 'train' first.");
        return;
    }

    let (r_squared, mse) = precision(&data, theta0, theta1);
    let rmse = mse.sqrt();
    
    println!("Model precision:");
    println!("  R² (coefficient of determination) = {:.4}", r_squared);
    println!("  MSE (mean squared error)          = {:.2}", mse);
    println!("  RMSE (root mean squared error)    = {:.2}€", rmse);
    println!();
    println!("R² interpretation: {:.1}% of price variance is explained by the model.", r_squared * 100.0);
    println!("RMSE interpretation: model is on average {:.2}€ off per prediction.", rmse);
}