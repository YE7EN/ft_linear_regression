use ft_linear_regression::{load_dataset, train, save_thetas};

fn main() {
    let data = load_dataset("data.csv").expect("Failed to load dataset");
    println!("Loaded {} data points", data.len());

    let learning_rate = 0.1;
    let iterations = 1000;

    let (theta0, theta1) = train(&data, learning_rate, iterations);

    println!("Training done.");
    println!("theta0 = {}", theta0);
    println!("theta1 = {}", theta1);

    save_thetas("thetas.txt", theta0, theta1).expect("Failed to save thetas");
    println!("Thetas saved to thetas.txt");
}