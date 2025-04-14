use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Xy {
    pub x: f64,
    pub y: f64,
}

fn distance(p1: &Xy, p2: &Xy) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn fitness_function(warehouse_positions: &[f64], stores: &[Xy], residential: &[Xy]) -> f64 {
    let warehouses: Vec<Xy> = warehouse_positions
        .chunks(2)
        .map(|c: &[f64]| Xy { x: c[0], y: c[1] })
        .collect();

    let min_store_dists: Vec<f64> = stores
        .iter()
        .map(|s| {
            warehouses
                .iter()
                .map(|w: &Xy| distance(w, s))
                .fold(f64::INFINITY, f64::min)
        })
        .collect();

    let min_res_dists: Vec<f64> = residential
        .iter()
        .map(|r| {
            warehouses
                .iter()
                .map(|w: &Xy| distance(w, r))
                .fold(f64::INFINITY, f64::min)
        })
        .collect();

    min_res_dists.iter().copied().fold(f64::INFINITY, f64::min)
        - min_store_dists
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
}

struct Particle {
    position: Vec<f64>,
    velocity: Vec<f64>,
    best_position: Vec<f64>,
    best_fitness: f64,
}

impl Particle {
    fn new(bounds: &[(f64, f64)], stores: &[Xy], residential: &[Xy]) -> Self {
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let position: Vec<f64> = bounds
            .iter()
            .map(|(lo, hi)| rng.random_range(*lo..=*hi))
            .collect::<Vec<_>>();
        let velocity: Vec<f64> = bounds
            .iter()
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect::<Vec<_>>();
        let fitness: f64 = fitness_function(&position, stores, residential);
        Particle {
            best_position: position.clone(),
            best_fitness: fitness,
            position,
            velocity,
        }
    }

    fn update_velocity(
        &mut self,
        global_best: &[f64],
        i_weight: f64,
        m_weight: f64,
        s_weight: f64,
    ) {
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        (0..self.velocity.len()).for_each(|i: usize| {
            let inertia: f64 = i_weight * self.velocity[i];
            let memory: f64 =
                m_weight * rng.random::<f64>() * (self.best_position[i] - self.position[i]);
            let social: f64 = s_weight * rng.random::<f64>() * (global_best[i] - self.position[i]);
            self.velocity[i] = inertia + memory + social;
        });
    }

    fn update_position(&mut self, bounds: &[(f64, f64)], stores: &[Xy], residential: &[Xy]) {
        (0..self.position.len()).for_each(|i: usize| {
            self.position[i] += self.velocity[i];
            let (lo, hi) = bounds[i];
            self.position[i] = self.position[i].clamp(lo, hi);
        });

        let fitness: f64 = fitness_function(&self.position, stores, residential);
        if fitness > self.best_fitness {
            self.best_fitness = fitness;
            self.best_position = self.position.clone();
        }
    }
}

pub struct Swarm<'a> {
    pub global_best_position: Vec<f64>,
    global_best_fitness: f64,
    particles: Vec<Particle>,
    bounds: Vec<(f64, f64)>,
    stores: &'a [Xy],
    residential: &'a [Xy],
    i_weight: f64,
    m_weight: f64,
    s_weight: f64,
}

impl<'a> Swarm<'a> {
    pub fn new(
        num_particles: usize,
        bounds: &[(f64, f64)],
        stores: &'a [Xy],
        residential: &'a [Xy],
        i_weight: f64,
        m_weight: f64,
        s_weight: f64,
    ) -> Self {
        let particles: Vec<Particle> = (0..num_particles)
            .map(|_| Particle::new(bounds, stores, residential))
            .collect::<Vec<_>>();

        let best: &Particle = &particles[0];
        Swarm {
            global_best_position: best.best_position.clone(),
            global_best_fitness: best.best_fitness,
            particles,
            bounds: bounds.to_vec(),
            stores,
            residential,
            i_weight,
            m_weight,
            s_weight,
        }
    }

    pub fn optimise(&mut self, iterations: usize) {
        for _ in 0..iterations {
            for p in &mut self.particles {
                p.update_velocity(
                    &self.global_best_position,
                    self.i_weight,
                    self.m_weight,
                    self.s_weight,
                );
                p.update_position(&self.bounds, self.stores, self.residential);

                if p.best_fitness > self.global_best_fitness {
                    println!("Fit: {}", p.best_fitness);
                    self.global_best_fitness = p.best_fitness;
                    self.global_best_position = p.best_position.clone();
                }
            }
        }
    }
}
