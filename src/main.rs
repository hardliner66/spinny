use notan::draw::*;
use notan::math::{Mat3, Vec2};
use notan::prelude::*;

const COLORS: [Color; 8] = [
    Color::WHITE,
    Color::MAGENTA,
    Color::ORANGE,
    Color::RED,
    Color::YELLOW,
    Color::AQUA,
    Color::MAROON,
    Color::PINK,
];

#[derive(AppState)]
struct State {
    font: Font,
    size: f32,
    speed: f32,
    segments: i32,
    rot: f32,
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
        .unwrap();
    State {
        font,
        segments: 6,
        size: 200.0,
        speed: 0.0,
        rot: 0.0,
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, _assets: &mut Assets, _plugins: &mut Plugins, state: &mut State) {
    if app.mouse.left_was_pressed() {
        if state.speed < 100.0 {
            state.speed += 200.0 + (notan::random::rand::random::<f32>() * 100.0);
        } else {
            state.speed += 50.0 + (notan::random::rand::random::<f32>() * 150.0);
        }
    }
    if state.speed > 400.0 {
        state.speed = 400.0;
    }
}

#[allow(dead_code)]
fn get_circumference(radius: f32) -> f32 {
    2.0 * std::f32::consts::PI * radius
}

fn get_angle(count: i32) -> f32 {
    360.0 / count as f32
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Push to the transformation stack a translation matrix
    draw.transform()
        .push(Mat3::from_translation(Vec2::new(350.0, 350.0)));

    let rotation = Mat3::from_angle(state.rot.to_radians());
    draw.transform().push(rotation);

    let angle = get_angle(state.segments).to_radians();

    let rotation = Mat3::from_angle(-(angle / 2.0));
    draw.transform().push(rotation);
    for i in 0..state.segments {
        let rotation = Mat3::from_angle(angle);
        draw.transform().push(rotation);
        draw.triangle(
            (0.0, 0.0),
            (0.0, state.size),
            (state.size * angle.sin(), state.size * angle.cos()),
        )
        .color(COLORS[usize::try_from(i).unwrap() % COLORS.len()]);
    }

    // Reset the transformation stack
    draw.transform().clear();

    // Push to the transformation stack a translation matrix
    draw.transform()
        .push(Mat3::from_translation(Vec2::new(350.0, 350.0)));

    let arrow_size = 50.0;
    let height = state.size * angle.sin();
    let p1 = (-(arrow_size / 2.0), -state.size - arrow_size);
    let p2 = (arrow_size / 2.0, -state.size - arrow_size);
    let p3 = (0.0, -height);
    draw.triangle(p1, p2, p3).color(Color::WHITE);
    draw.line(p1, p2).width(2.0).color(Color::BLACK);
    draw.line(p2, p3).width(2.0).color(Color::BLACK);
    draw.line(p3, p1).width(2.0).color(Color::BLACK);

    // Reset the transformation stack
    draw.transform().clear();

    draw.text(&state.font, &format!("{}", (state.rot / angle.to_degrees()).round()))
        .position(50.0, 10.0)
        .size(30.0)
        .color(Color::WHITE)
        .h_align_center()
        .v_align_middle();

    // Render the frame
    gfx.render(&draw);

    if state.speed > 0.0 {
        state.rot = (state.rot + app.timer.delta_f32() * state.speed) % 360.0;
        state.speed -= app.timer.delta_f32() * 50.0;
    }
    if state.speed < 0.1 {
        state.speed = 0.0;
    }
}
