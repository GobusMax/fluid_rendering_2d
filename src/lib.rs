use macroquad::{
    prelude::{dvec2, DVec2},
    rand::gen_range,
};

pub fn p_norm(values: &[f64], p: f64) -> f64 {
    values
        .iter()
        .fold(0., |a, v| a + v.abs().powf(p))
        .powf(1. / p)
}

pub fn boltzman(values: &[f64], alpha: f64) -> f64 {
    values.iter().fold(0., |a, v| a + v * (alpha * v).exp())
        / values.iter().fold(0., |a, v| a + (alpha * v).exp())
}
pub fn log_sum_exp(values: &[f64], alpha: f64) -> f64 {
    values.iter().fold(0., |a, v| a + (alpha * v).exp()).ln() / alpha
}
pub fn mellowmax(values: &[f64], alpha: f64) -> f64 {
    (values.iter().fold(0., |a, v| a + (alpha * v).exp()) / values.len() as f64).ln() / alpha
}
pub fn random_vec(low: DVec2, high: DVec2) -> DVec2 {
    dvec2(gen_range(low.x, high.x), gen_range(low.y, high.y))
}
