use ft_linear_regression::{load_dataset, load_thetas, estimate_price};
use plotters::prelude::*;

fn main() {
    let data = load_dataset("data.csv").expect("Failed to load dataset");
    let (theta0, theta1) = load_thetas("thetas.txt");

    let km_min = data.iter().map(|p| p.mileage).fold(f64::INFINITY, f64::min);
    let km_max = data.iter().map(|p| p.mileage).fold(f64::NEG_INFINITY, f64::max);
    let price_min = data.iter().map(|p| p.price).fold(f64::INFINITY, f64::min);
    let price_max = data.iter().map(|p| p.price).fold(f64::NEG_INFINITY, f64::max);

    let km_margin = (km_max - km_min) * 0.1;
    let price_margin = (price_max - price_min) * 0.1;

    let output = "plot.png";
    let root = BitMapBackend::new(output, (900, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Car price vs mileage — Linear Regression", ("sans-serif", 22))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (km_min - km_margin)..(km_max + km_margin),
            (price_min - price_margin)..(price_max + price_margin),
        )
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Mileage (km)")
        .y_desc("Price (€)")
        .draw()
        .unwrap();

    chart
        .draw_series(
            data.iter().map(|p| {
                Circle::new((p.mileage, p.price), 6, BLUE.filled())
            }),
        )
        .unwrap()
        .label("Data points")
        .legend(|(x, y)| Circle::new((x, y), 5, BLUE.filled()));

    let line_points = vec![
        (km_min, estimate_price(km_min, theta0, theta1)),
        (km_max, estimate_price(km_max, theta0, theta1)),
    ];
    chart
        .draw_series(LineSeries::new(line_points, RED.stroke_width(2)))
        .unwrap()
        .label("Linear regression")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(2)));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
    println!("Plot saved to {}", output);
}