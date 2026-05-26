use ft_linear_regression::load_dataset;

fn main()
{
    let data = load_dataset("data.csv").expect("Failed to load dataset");


    println!("Loaded {} data points", data.len());
    for point in data.iter().take(5)
    {
        println!(" mileage = {}, price = {}", point.mileage, point.price);
    }
}
