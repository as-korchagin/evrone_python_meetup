use ndarray::prelude::*;
use pyo3::prelude::*;
use rand::Rng;
use std::f64::INFINITY;

fn path_durations(
    matrix: &Array<f64, Dim<[usize; 2]>>,
    paths: &Array<i64, Dim<[usize; 2]>>,
    matrix_size: usize,
) -> Vec<f64> {
    let mut durations = Vec::new();

    for path_idx in 0..matrix_size {
        let mut duration = 0.0;
        for step_idx in 1..matrix_size {
            let start = paths[[path_idx, step_idx - 1]] as usize;
            let finish = paths[[path_idx, step_idx]] as usize;
            duration += matrix[[start, finish]];
        }
        durations.push(duration);
    }

    durations
}

fn generate_paths(
    matrix: &Array<f64, Dim<[usize; 2]>>,
    places: &Vec<usize>,
    pheromone: &Array<f64, Dim<[usize; 2]>>,
    heuristic: &Array<f64, Dim<[usize; 2]>>,
    matrix_size: usize,
    alpha: f64,
    beta: f64,
) -> Array<i64, Dim<[usize; 2]>> {
    let mut ants_path = Array::<i64, _>::zeros([matrix_size, matrix_size]) - 1;
    let matrix_size = matrix.shape()[0];
    let mut rng = rand::thread_rng();

    for i in 0..matrix_size {
        ants_path[[i, 0]] = 0;
        ants_path[[i, matrix_size - 1]] = matrix_size as i64 - 1;
    }

    for ant in 0..matrix_size {
        for place in 1..matrix_size - 1 {
            let mut unvisited = Vec::new();
            let ant_vec: Vec<usize> = ants_path
                .index_axis(Axis(0), ant)
                .into_iter()
                .filter(|&x| *x >= 0)
                .map(|&x| x as usize)
                .collect();

            for place_idx in places {
                if !ant_vec.contains(place_idx) {
                    unvisited.push(place_idx)
                }
            }

            if unvisited.len() == 1 {
                ants_path[[ant, place]] = *unvisited[0] as i64;
                continue;
            }

            let mut p_total: f64 = 0.0;
            let from = ants_path[[ant, place - 1]] as usize;
            for &u_point in unvisited.iter() {
                p_total += pheromone[[from, *u_point]].powf(alpha)
                    * heuristic[[from, *u_point]].powf(beta);
            }
            let mut p_by_unvisited = Array::<f64, _>::zeros([1, unvisited.len()]);

            for (u_idx, &u_point) in unvisited.iter().enumerate() {
                p_by_unvisited[[0, u_idx]] = pheromone[[from, *u_point]].powf(alpha)
                    * heuristic[[from, *u_point]].powf(beta)
                    / p_total;
            }

            let cumsum: Vec<f64> = p_by_unvisited
                .iter()
                .scan(0.0, |acc, &x| {
                    *acc = *acc + x;
                    Some(*acc)
                })
                .collect();
            let random: f64 = rng.gen_range(0.0..1.0);
            for (u_idx, &cumsum_item) in cumsum.iter().enumerate() {
                if cumsum_item >= random {
                    ants_path[[ant, place]] = *unvisited[u_idx] as i64;
                    break;
                }
            }
        }
    }

    ants_path
}

#[pyfunction]
fn optimize(d_matrix: Vec<Vec<f64>>, iterations: usize) -> PyResult<Vec<i64>> {
    let matrix_size = d_matrix.len();
    let mut matrix = Array2::<f64>::default((matrix_size, matrix_size));
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            matrix[[i, j]] = d_matrix[i][j];
        }
    }
    let (alpha, beta, phe_eva_rate) = (1.0, 1.0, 0.3);
    let mut pheromone = Array::<f64, _>::ones([matrix_size, matrix_size]);
    let places = (0..matrix_size).collect();
    let heuristic =
        1.0 / (Array::<f64, _>::eye(matrix_size) + &matrix) - Array::<f64, _>::eye(matrix_size);
    let mut min_duration = INFINITY;
    let mut min_route: Vec<i64> = vec![1];

    for _ in 0..iterations {
        let generated_paths = generate_paths(
            &matrix,
            &places,
            &pheromone,
            &heuristic,
            matrix_size,
            alpha,
            beta,
        );
        pheromone *= 1.0 - phe_eva_rate;
        let durations = path_durations(&matrix, &generated_paths, matrix_size);
        for path_idx in 0..matrix_size {
            let delta_tau = 1.0 / durations[path_idx] as f64;
            for place_idx in 1..matrix_size {
                let start = generated_paths[[path_idx, place_idx - 1]] as usize;
                let finish = generated_paths[[path_idx, place_idx]] as usize;
                pheromone[[start, finish]] += delta_tau;
            }
        }

        for (d_idx, &duration) in durations.iter().enumerate() {
            if duration < min_duration {
                min_duration = duration;
                min_route = generated_paths.index_axis(Axis(0), d_idx).to_vec();
            }
        }
    }
    Ok(min_route)
}

#[pymodule]
fn ant_colony_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(optimize, m)?)?;
    Ok(())
}
