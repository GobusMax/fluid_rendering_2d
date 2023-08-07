use fluid_rendering_2d::{mellowmax, random_vec};
use macroquad::prelude::*;

const FOV: f64 = 90.;
const NUM_RAYS: usize = 100;
const STOPPING_DISTANCE: f64 = 10.;
#[macroquad::main("BasicShapes")]
async fn main() {
    let mut points = vec![DVec2::NAN; 100];
    let mut pos = DVec2::ZERO;

    for v in &mut points {
        *v = random_vec(
            vec2(screen_width() / 4., screen_width() / 4.).as_dvec2(),
            vec2(screen_width() * 3. / 4., screen_height() * 3. / 4.).as_dvec2(),
        );
    }
    loop {
        clear_background(WHITE);
        if is_mouse_button_pressed(MouseButton::Left) {
            pos = Vec2::from(mouse_position()).as_dvec2();
        }
        for v in &points {
            draw_circle(v.x as f32, v.y as f32, STOPPING_DISTANCE as f32, RED);
        }
        let dir = (Vec2::from(mouse_position()).as_dvec2() - pos).normalize();

        let res = multi_cast(pos, dir, &points, STOPPING_DISTANCE);
        for (i, v) in res.iter().enumerate() {
            let color = if !v.is_nan() {
                draw_circle(v.x as f32, v.y as f32, 5., GREEN);
                let dist_scaled = 1. - (v.distance(pos) / 100.).tanh();
                Color::new(
                    dist_scaled as f32,
                    dist_scaled as f32,
                    dist_scaled as f32,
                    1.,
                )
            } else {
                RED
            };
            draw_rectangle(
                i as f32 * screen_width() / NUM_RAYS as f32,
                screen_height() - 32.,
                screen_width() / NUM_RAYS as f32,
                32.,
                color,
            )
        }
        next_frame().await
    }
}

fn sphere_tracing(
    start: DVec2,
    dir: DVec2,
    points: &[DVec2],
    stopping_distance: f64,
    draw_circles: bool,
) -> DVec2 {
    let mut pos = start;
    let dir_normalized = dir.normalize_or_zero();
    draw_line(
        start.x as f32,
        start.y as f32,
        (start.x + dir_normalized.x * 1000.) as f32,
        (start.y + dir_normalized.y * 1000.) as f32,
        1.,
        BLACK,
    );
    for _ in 0..100 {
        let mut min_dist = f64::INFINITY;
        for p in points {
            let dist = p.distance(pos);
            if dist < min_dist {
                min_dist = dist;
            }
        }
        // let diff: Vec<_> = points.iter().map(|v| v.distance(pos)).collect();
        // min_dist = mellowmax(&diff, -0.1);
        if draw_circles {
            draw_circle_lines(pos.x as f32, pos.y as f32, min_dist as f32, 1., GREEN);
        }
        if min_dist <= stopping_distance * 1.001 {
            return pos;
        } else {
            pos += dir_normalized * (min_dist - stopping_distance);
        }
    }
    draw_line(
        (start.x) as f32,
        (start.y) as f32,
        (start.x + dir_normalized.x * 1000.) as f32,
        (start.y + dir_normalized.y * 1000.) as f32,
        1.,
        RED,
    );
    DVec2::NAN
}

fn multi_cast(start: DVec2, dir: DVec2, points: &[DVec2], stopping_distance: f64) -> Vec<DVec2> {
    let mut res = vec![];

    let forward = dir.normalize();
    let right = -forward.perp();
    let tranform = dmat2(forward, right);
    let dist = 0.5 / (FOV * 0.5).to_radians().tan();

    for i in 0..NUM_RAYS {
        let ray_dir = if NUM_RAYS > 1 {
            tranform.mul_vec2(dvec2(dist, (i as f64 / (NUM_RAYS - 1) as f64) - 0.5).normalize())
        } else {
            forward
        };
        let intersection = sphere_tracing(start, ray_dir, points, stopping_distance, true);
        res.push(intersection);
    }

    res
}
