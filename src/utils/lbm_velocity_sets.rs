// src/utils/lbm_velocity_sets.rs

#[derive(Debug)]
pub struct VelocitySet {
    pub c: Vec<Vec<i32>>, // Velocity vectors
    pub w: Vec<f32>,      // Weights
}

pub fn get_velocity_set(set_name: &str) -> Option<VelocitySet> {
    match set_name {
        "D2Q9" => Some(VelocitySet {
            c: vec![
                vec![0, 0, 0], vec![1, 0, 0], vec![-1, 0, 0], vec![0, 1, 0], vec![0, -1, 0],
                vec![1, 1, 0], vec![-1, -1, 0], vec![1, -1, 0], vec![-1, 1, 0],
            ],
            w: vec![
                4.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0,
                1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
            ],
        }),
        "D3Q7" => Some(VelocitySet {
            c: vec![
                vec![0, 0, 0], vec![1, 0, 0], vec![-1, 0, 0], vec![0, 1, 0],
                vec![0, -1, 0], vec![0, 0, 1], vec![0, 0, -1],
            ],
            w: vec![1.0 / 4.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0],
        }),
        "D3Q15" => Some(VelocitySet {
            c: vec![
                vec![0, 0, 0], vec![1, 0, 0], vec![-1, 0, 0], vec![0, 1, 0],
                vec![0, -1, 0], vec![0, 0, 1], vec![0, 0, -1], vec![1, 1, 1],
                vec![-1, -1, -1], vec![1, 1, -1], vec![-1, -1, 1], vec![1, -1, 1],
                vec![-1, 1, -1], vec![-1, 1, 1], vec![1, -1, -1],
            ],
            w: vec![
                2.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0,
                1.0 / 9.0, 1.0 / 9.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0,
                1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0, 1.0 / 72.0,
            ],
        }),
        "D3Q19" => Some(VelocitySet {
            c: vec![
                vec![0, 0, 0], vec![1, 0, 0], vec![-1, 0, 0], vec![0, 1, 0],
                vec![0, -1, 0], vec![0, 0, 1], vec![0, 0, -1], vec![1, 1, 0],
                vec![-1, -1, 0], vec![1, 0, 1], vec![-1, 0, -1], vec![0, 1, 1],
                vec![0, -1, -1], vec![1, -1, 0], vec![-1, 1, 0], vec![1, 0, -1],
                vec![-1, 0, 1], vec![0, 1, -1], vec![0, -1, 1],
            ],
            w: vec![
                1.0 / 3.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0,
                1.0 / 18.0, 1.0 / 18.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
                1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
                1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
            ],
        }),
        "D3Q27" => Some(VelocitySet {
            c: vec![
                vec![0, 0, 0], vec![1, 0, 0], vec![-1, 0, 0], vec![0, 1, 0],
                vec![0, -1, 0], vec![0, 0, 1], vec![0, 0, -1], vec![1, 1, 0],
                vec![-1, -1, 0], vec![1, 0, 1], vec![-1, 0, -1], vec![0, 1, 1],
                vec![0, -1, -1], vec![1, -1, 0], vec![-1, 1, 0], vec![1, 0, -1],
                vec![-1, 0, 1], vec![0, 1, -1], vec![0, -1, 1], vec![1, 1, 1],
                vec![-1, -1, -1], vec![1, 1, -1], vec![-1, -1, 1], vec![1, -1, 1],
                vec![-1, 1, -1], vec![-1, 1, 1], vec![1, -1, -1],
            ],
            w: vec![
                8.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0, 2.0 / 27.0,
                2.0 / 27.0, 2.0 / 27.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0,
                1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0,
                1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 54.0, 1.0 / 216.0,
                1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0, 1.0 / 216.0,
                1.0 / 216.0, 1.0 / 216.0,
            ],
        }),
        _ => None, // Handle unknown velocity sets
    }
}

