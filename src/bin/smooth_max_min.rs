use fluid_rendering_2d::{boltzman, log_sum_exp, log_sum_exp_grad, mellowmax, random_vec};
use macroquad::prelude::*;

const RADIUS: f32 = 4.;
const MIN_DISTANCE: f64 = 32.;

#[derive(Debug)]
enum MinFunction {
    Boltzman,
    LogSumExp,
    Mellowmax,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points = vec![DVec2::NAN; 10];

    let mut image = Image::gen_image_color(screen_width() as _, screen_height() as _, WHITE);

    for v in &mut points {
        *v = random_vec(
            vec2(screen_width() / 4., screen_width() / 4.).as_dvec2(),
            vec2(screen_width() * 3. / 4., screen_height() * 3. / 4.).as_dvec2(),
        );
    }
    let mut alpha: f64 = -0.1;
    let mut min_distance: f64 = 32.;
    let mut smooth_function = MinFunction::Boltzman;
    loop {
        clear_background(WHITE);
        image = Image::gen_image_color(screen_width() as _, screen_height() as _, WHITE);
        points[0] = Vec2::from(mouse_position()).as_dvec2();
        if is_key_down(KeyCode::Down) {
            alpha -= 0.01;
            alpha = alpha.clamp(-4., 0.);
        }
        if is_key_down(KeyCode::Up) {
            alpha += 0.01;
            alpha = alpha.clamp(-4., 0.);
        }
        if is_key_down(KeyCode::Left) {
            min_distance -= 1.;
        }
        if is_key_down(KeyCode::Right) {
            min_distance += 1.;
        }
        if is_key_pressed(KeyCode::Key1) {
            smooth_function = MinFunction::Boltzman
        }
        if is_key_pressed(KeyCode::Key2) {
            smooth_function = MinFunction::LogSumExp
        }
        if is_key_pressed(KeyCode::Key3) {
            smooth_function = MinFunction::Mellowmax
        }
        if is_key_pressed(KeyCode::R) {
            for v in &mut points {
                *v = random_vec(
                    vec2(screen_width() / 4., screen_width() / 4.).as_dvec2(),
                    vec2(screen_width() * 3. / 4., screen_height() * 3. / 4.).as_dvec2(),
                );
            }
        }

        for x in 0..screen_width() as usize {
            for y in 0..screen_height() as usize {
                let pos = dvec2(x as f64, y as f64);
                let diff: Vec<_> = points.iter().map(|v| *v - pos).collect();
                let distances: Vec<_> = diff.iter().map(|v| v.length()).collect();

                let res = match smooth_function {
                    MinFunction::Boltzman => boltzman(&distances, alpha),
                    MinFunction::LogSumExp => log_sum_exp(&distances, alpha),
                    MinFunction::Mellowmax => mellowmax(&distances, alpha),
                };
                let grad = log_sum_exp_grad(&diff, alpha);
                if res <= min_distance {
                    image.set_pixel(x as _, y as _, GREEN);
                } else {
                    image.set_pixel(
                        x as _,
                        y as _,
                        Color {
                            r: grad.x as _,
                            g: grad.y as _,
                            b: -grad.x as _,
                            a: 1.,
                        },
                    );
                }
            }
        }
        let texture = Texture2D::from_image(&image);
        draw_texture(&texture, 0., 0., WHITE);
        for v in &points {
            draw_circle(v.x as _, v.y as _, RADIUS, RED);
        }
        draw_text(format!("Alpha: {}", alpha).as_str(), 10., 10., 16., BLACK);
        draw_text(
            format!("Min Distance: {}", min_distance).as_str(),
            10.,
            26.,
            16.,
            BLACK,
        );
        draw_text(
            format!("Smooth Function: {:?}", smooth_function).as_str(),
            10.,
            42.,
            16.,
            BLACK,
        );
        println!("{}", 1. / get_frame_time());
        next_frame().await
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "Window name".to_owned(),
        fullscreen: false,
        window_width: 256,
        window_height: 256,
        ..Default::default()
    }
}
