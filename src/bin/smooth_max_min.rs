use fluid_rendering_2d::{boltzman, log_sum_exp, mellowmax, random_vec};
use macroquad::prelude::*;

const RADIUS: f32 = 4.;
const MIN_DISTANCE: f64 = 32.;

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
    loop {
        clear_background(WHITE);
        image = Image::gen_image_color(screen_width() as _, screen_height() as _, WHITE);
        points[0] = Vec2::from(mouse_position()).as_dvec2();
        for x in 0..screen_width() as usize {
            for y in 0..screen_height() as usize {
                let diff: Vec<_> = points
                    .iter()
                    .map(|v| v.distance(dvec2(x as _, y as _)))
                    .collect();
                let res = mellowmax(&diff, -0.1);
                if res <= MIN_DISTANCE {
                    image.set_pixel(x as _, y as _, GREEN);
                }
            }
        }
        let texture = Texture2D::from_image(&image);
        draw_texture(&texture, 0., 0., WHITE);
        for v in &points {
            draw_circle(v.x as _, v.y as _, RADIUS, RED);
        }
        next_frame().await
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "Window name".to_owned(),
        fullscreen: false,
        window_width: 200,
        window_height: 200,
        ..Default::default()
    }
}
