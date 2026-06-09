use plotters::prelude::*;

fn main() {
    let content = match std::fs::read_to_string("costs.txt") {
    Ok(c) => c,
    Err(_) => {
        println!("Could not read costs.txt — run train first");
        return;
    }
};
    
    let costs: Vec<f64> = content
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let max_cost = costs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_cost = costs.iter().cloned().fold(f64::INFINITY, f64::min);

    let output = "plot_loss.png";
    let root = BitMapBackend::new(output, (900, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Gradient Descent — Cost over iterations", ("sans-serif", 22))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(0..costs.len(), min_cost..max_cost)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Iterations")
        .y_desc("Cost")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            costs.iter().enumerate().map(|(i, &c)| (i, c)),
            RED.stroke_width(2),
        ))
        .unwrap()
        .label("Cost")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(2)));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
    println!("Loss curve saved to {}", output);
}