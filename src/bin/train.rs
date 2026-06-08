use ft_linear_regression::{load_dataset, train, save_thetas};

fn main() {
    let data = match load_dataset("data.csv") {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Error: could not load data.csv");
            eprintln!("Make sure data.csv is in the current directory.");
            return;
        }
    };

    println!("Loaded {} data points", data.len());

    let learning_rate = 0.1;
    let iterations = 1000;

    let (theta0, theta1) = train(&data, learning_rate, iterations);

    println!("Training done.");
    println!("theta0 = {}", theta0);
    println!("theta1 = {}", theta1);

    match save_thetas("thetas.txt", theta0, theta1) {
        Ok(_) => println!("Thetas saved to thetas.txt"),
        Err(_) => eprintln!("Error: could not save thetas.txt"),
    }
}