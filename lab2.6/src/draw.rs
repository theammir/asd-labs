use std::f32::consts::PI;

use raylib::prelude::*;

const VERTEX_FONT_SIZE: i32 = 32;
const WEIGHT_FONT_SIZE: i32 = 24;

const VERTEX_RADIUS: f32 = 20.0;
const VERTEX_WIDTH: u32 = 3;

const EDGE_BASE_ANGLE: f32 = 0.05 * PI;
const WEIGHT_TEXT_OFFSET: f32 = 0.4; // 0.0..=0.5

pub fn draw_text_pro(
    d: &mut RaylibDrawHandle,
    font: &Font,
    text: &str,
    position: Vector2,
    rotation: f32,
    font_size: f32,
    color: Color,
) {
    d.draw_text_pro(
        font,
        text,
        position,
        Vector2 {
            x: font.measure_text(text, font_size, 0.0).x * 0.5,
            y: 0.0,
        },
        rotation * 180.0 / PI,
        font_size,
        0.0,
        color,
    );
}

pub fn draw_text(
    d: &mut RaylibDrawHandle,
    font: &Font,
    text: &str,
    position: Vector2,
    font_size: f32,
    color: Color,
) {
    draw_text_pro(d, font, text, position, 0.0, font_size, color);
}

pub fn draw_vertex(
    d: &mut RaylibDrawHandle,
    center: Vector2,
    weight: &str,
    font: &Font,
    color: Color,
) {
    (0..VERTEX_WIDTH).for_each(|w| {
        d.draw_circle_lines(
            center.x as i32,
            center.y as i32,
            VERTEX_RADIUS - w as f32,
            color,
        );
    });
    if !weight.is_empty() {
        let y_offset: f32 = VERTEX_FONT_SIZE as f32 * 0.5;
        draw_text(
            d,
            font,
            weight,
            Vector2 {
                x: center.x,
                y: center.y - y_offset,
            },
            VERTEX_FONT_SIZE as f32,
            Color::BLACK,
        )
    }
}

fn draw_arrowhead(d: &mut RaylibDrawHandle, position: Vector2, direction: Vector2, color: Color) {
    const ARROWHEAD_LEN: f32 = VERTEX_RADIUS * 0.5;
    const ARROWHEAD_ANGLE: f32 = PI / 6.0;
    d.draw_line_v(
        position,
        position - direction.rotated(ARROWHEAD_ANGLE) * ARROWHEAD_LEN,
        color,
    );
    d.draw_line_v(
        position,
        position - direction.rotated(-ARROWHEAD_ANGLE) * ARROWHEAD_LEN,
        color,
    );
}

pub fn draw_edge_weight(
    d: &mut RaylibDrawHandle,
    center_from: Vector2,
    center_to: Vector2,
    weight: &str,
    font: &Font,
    color: Color,
    angled: bool,
) {
    let direction = center_to - center_from;
    let normal = direction.normalized().rotated(0.5 * PI);
    let offset = direction * WEIGHT_TEXT_OFFSET;

    let mut text_pos = center_from + offset;
    let mut angle = Vector2 { x: 1.0, y: 0.0 }.angle_to(direction);
    if angled {
        text_pos += normal * (offset.length() * EDGE_BASE_ANGLE.tan());
        angle = Vector2 { x: 1.0, y: 0.0 }.angle_to(text_pos - center_from);
    }
    draw_text_pro(
        d,
        font,
        weight,
        text_pos,
        angle,
        WEIGHT_FONT_SIZE as f32,
        color,
    );
}

pub fn draw_straight_edge(
    d: &mut RaylibDrawHandle,
    center_from: Vector2,
    center_to: Vector2,
    directional: bool,
    color: Color,
) {
    let direction = (center_to - center_from).normalized();

    let from = center_from + direction * VERTEX_RADIUS;
    let to = center_to - direction * VERTEX_RADIUS;
    d.draw_line_v(from, to, color);
    if directional {
        draw_arrowhead(d, to, direction, color);
    }
}

pub fn draw_angled_edge(
    d: &mut RaylibDrawHandle,
    center_from: Vector2,
    center_to: Vector2,
    directed: bool,
    color: Color,
) {
    let direction = (center_to - center_from).normalized();
    let from = center_from + direction * VERTEX_RADIUS;
    let to = center_to - direction * VERTEX_RADIUS;

    let vector = to - from;
    let vector_middle = vector * 0.5;
    let mid_offset = vector_middle.length() * EDGE_BASE_ANGLE.tan();

    let midpoint = from + vector_middle + direction.rotated(0.5 * PI) * mid_offset;
    d.draw_line_v(from, midpoint, color);
    d.draw_line_v(midpoint, to, color);
    if directed {
        draw_arrowhead(d, to, (to - midpoint).normalized(), color);
    }
}

pub fn draw_looping_edge(d: &mut RaylibDrawHandle, center: Vector2, color: Color) {
    const POINTS: usize = 16;
    const RADIUS: f32 = 12.0;
    const START_ANGLE: f32 = -0.9 * PI;
    const END_ANGLE: f32 = 0.75 * PI;

    let step = (END_ANGLE - START_ANGLE) / POINTS as f32;
    let start_point = center
        + Vector2 {
            x: VERTEX_RADIUS,
            y: -0.85 * VERTEX_RADIUS,
        };

    let points: [Vector2; POINTS] = std::array::from_fn(|i| Vector2 {
        x: start_point.x + RADIUS * f32::cos(START_ANGLE + (step * i as f32)),
        y: start_point.y + RADIUS * f32::sin(START_ANGLE + (step * i as f32)),
    });
    let last_point = points[POINTS - 1];
    d.draw_line_strip(&points, color);

    let direction = (last_point - points[POINTS - 4]).normalized();
    draw_arrowhead(d, last_point, direction, color);
}
