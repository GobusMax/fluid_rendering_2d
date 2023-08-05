use macroquad::prelude::{rand::gen_range, *};

const FOV: f32 = 90.;
const NUM_RAYS: usize = 100;
#[macroquad::main("BasicShapes")]
async fn main() {
    let mut points = vec![Vec2::NAN; 100];
    let mut pos = Vec2::ZERO;

    for v in &mut points {
        *v = random_vec(
            vec2(screen_width() / 4., screen_width() / 4.),
            vec2(screen_width() * 3. / 4., screen_height() * 3. / 4.),
        );
    }
    loop {
        clear_background(WHITE);
        if is_mouse_button_pressed(MouseButton::Left) {
            pos = mouse_position().into();
        }
        for v in &points {
            draw_circle(v.x, v.y, 5., RED);
        }
        let dir = (Vec2::from(mouse_position()) - pos).normalize();

        let res = multi_cast(pos, dir, &points, 10.);
        for (i, v) in res.iter().enumerate() {
            let color = if !v.is_nan() {
                draw_circle(v.x, v.y, 5., GREEN);
                let dist_scaled = 1. - (v.distance(pos) / 100.).tanh();
                Color::new(dist_scaled, dist_scaled, dist_scaled, 1.)
            } else {
                println!("WOWZERS");
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

fn random_vec(low: Vec2, high: Vec2) -> Vec2 {
    vec2(gen_range(low.x, high.x), gen_range(low.y, high.y))
}

fn sphere_tracing(
    start: Vec2,
    dir: Vec2,
    points: &Vec<Vec2>,
    stopping_distance: f32,
    draw_circles: bool,
) -> Vec2 {
    let mut pos = start;
    let dir_normalized = dir.normalize_or_zero();
    draw_line(
        start.x,
        start.y,
        start.x + dir_normalized.x * 1000.,
        start.y + dir_normalized.y * 1000.,
        1.,
        BLACK,
    );
    for _ in 0..100 {
        let mut min_dist = f32::INFINITY;
        for p in points {
            let dist = p.distance(pos);
            if dist < min_dist {
                min_dist = dist;
            }
        }
        if draw_circles {
            draw_circle_lines(pos.x, pos.y, min_dist, 1., GREEN);
        }
        if min_dist <= stopping_distance * 1.001 {
            return pos;
        } else {
            pos += dir_normalized * (min_dist - stopping_distance);
        }
    }

    Vec2::NAN
}

fn multi_cast(start: Vec2, dir: Vec2, points: &Vec<Vec2>, stopping_distance: f32) -> Vec<Vec2> {
    let mut res = vec![];

    let forward = dir.normalize();
    let right = -forward.perp();
    let tranform = mat2(forward, right);
    let dist = 0.5 / (FOV * 0.5).to_radians().tan();

    for i in 0..NUM_RAYS {
        let ray_dir = tranform.mul_vec2(vec2(dist, (i as f32 / NUM_RAYS as f32) - 0.5).normalize());
        let intersection = sphere_tracing(start, ray_dir, points, stopping_distance, false);
        res.push(intersection);
    }
    res
}
