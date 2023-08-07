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

pub fn log_sum_exp_grad(points: &[DVec2], alpha: f64) -> DVec2 {
    let distances = points.iter().map(|v| v.length());
    let sum_exp: f64 = distances.clone().map(|v| (v * alpha).exp()).sum();
    let partial_derivatives = distances.map(|v| (v * alpha).exp() / sum_exp);
    let dist_gradients = points.iter().map(|v| -v.normalize());
    let gradient: DVec2 = dist_gradients
        .zip(partial_derivatives)
        .map(|(pd, dg)| dg * pd)
        .fold(DVec2::ZERO, |a, v| a + v);
    gradient
}
pub fn mellowmax(values: &[f64], alpha: f64) -> f64 {
    (values.iter().fold(0., |a, v| a + (alpha * v).exp()) / values.len() as f64).ln() / alpha
}
pub fn random_vec(low: DVec2, high: DVec2) -> DVec2 {
    dvec2(gen_range(low.x, high.x), gen_range(low.y, high.y))
}
