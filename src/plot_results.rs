use plotters::prelude::*;

pub fn plot_results(
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    store_locations: &[(i32, i32)],
    residential_locations: &[(i32, i32)],
    warehouse_locations: &[(i32, i32)],
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&RGBColor(0, 0, 0))?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Optimized Warehouse Locations",
            ("sans-serif", 30)
                .into_font()
                .color(&RGBColor(223, 223, 223)),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_bounds.0..x_bounds.1, y_bounds.0..y_bounds.1)?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .max_light_lines(4)
        .light_line_style(RGBColor(69, 64, 92).mix(0.7))
        .axis_style(RGBColor(69, 64, 92))
        .x_label_style(
            ("sans-serif", 15)
                .into_font()
                .color(&RGBColor(223, 223, 223)),
        )
        .y_label_style(
            ("sans-serif", 15)
                .into_font()
                .color(&RGBColor(223, 223, 223)),
        )
        .draw()?;

    // Plot stores
    for &(x, y) in store_locations {
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, y),
                5,
                RGBColor(106, 255, 155).filled(),
            )))?
            .label("Store")
            .legend(|(x, y)| Circle::new((x, y), 5, RGBColor(106, 255, 155).filled()));
    }

    // Plot residential areas
    for &(x, y) in residential_locations {
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, y),
                5,
                RGBColor(249, 126, 114).filled(),
            )))?
            .label("Residential")
            .legend(|(x, y)| Circle::new((x, y), 5, RGBColor(249, 126, 114).filled()));
    }

    // Plot warehouses
    for &(x, y) in warehouse_locations {
        chart
            .draw_series(std::iter::once(Circle::new((x, y), 6, WHITE.filled())))?
            .label("Warehouse")
            .legend(|(x, y)| Circle::new((x, y), 6, WHITE.filled()));
    }

    // Add legend
    chart
        .configure_series_labels()
        .background_style(RGBColor(23, 21, 32))
        .border_style(RGBColor(223, 223, 223))
        .label_font(
            ("sans-serif", 15)
                .into_font()
                .color(&RGBColor(223, 223, 223)),
        )
        .draw()?;

    Ok(())
}
