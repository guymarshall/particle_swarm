mod optimiser;
mod plot_results;

use optimiser::{Swarm, Xy};
use plot_results::plot_results;

fn main() {
    let x_bounds: (f64, f64) = (0.0, 100.0);
    let y_bounds: (f64, f64) = (0.0, 100.0);
    let num_warehouses: i32 = 3;
    let num_particles: usize = 500;
    let num_iterations: usize = 1000;
    let i_weight: f64 = 0.7;
    let m_weight: f64 = 1.5;
    let s_weight: f64 = 1.5;

    let store_locations: Vec<Xy> = vec![
        Xy { x: 20.0, y: 20.0 },
        Xy { x: 20.0, y: 80.0 },
        Xy { x: 80.0, y: 20.0 },
        Xy { x: 80.0, y: 80.0 },
    ];

    let residential_locations: Vec<Xy> = vec![
        Xy { x: 15.0, y: 10.0 },
        Xy { x: 30.0, y: 90.0 },
        Xy { x: 10.0, y: 35.0 },
        Xy { x: 50.0, y: 40.0 },
    ];

    // Flattened bounds for each warehouse
    let mut bounds: Vec<(f64, f64)> = Vec::new();
    for _ in 0..num_warehouses {
        bounds.push(x_bounds);
        bounds.push(y_bounds);
    }

    let mut swarm: Swarm<'_> = Swarm::new(
        num_particles,
        &bounds,
        &store_locations,
        &residential_locations,
        i_weight,
        m_weight,
        s_weight,
    );
    swarm.optimise(num_iterations);

    let warehouse_locations: Vec<Xy> = swarm
        .global_best_position
        .chunks(2)
        .map(|chunk: &[f64]| Xy {
            x: chunk[0],
            y: chunk[1],
        })
        .collect::<Vec<_>>();

    let x_bounds_int: (i32, i32) = (x_bounds.0 as i32, x_bounds.1 as i32);
    let y_bounds_int: (i32, i32) = (y_bounds.0 as i32, y_bounds.1 as i32);

    let to_int = |v: &[Xy]| {
        v.iter()
            .map(|p: &Xy| (p.x as i32, p.y as i32))
            .collect::<Vec<_>>()
    };

    if let Err(e) = plot_results(
        x_bounds_int,
        y_bounds_int,
        &to_int(&store_locations),
        &to_int(&residential_locations),
        &to_int(&warehouse_locations),
    ) {
        eprintln!("Plot error: {}", e);
    }
}
