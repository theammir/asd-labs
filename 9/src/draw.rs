use std::f32::consts::PI;

use raylib::prelude::*;

const FONT_SIZE: i32 = 32;
const CHAR_WIDTH: f32 = 0.27;

const VERTEX_RADIUS: f32 = 20.0;

const ARROWHEAD_LEN: f32 = VERTEX_RADIUS * 0.5;
const ARROWHEAD_ANGLE: f32 = PI / 6.0;

pub fn draw_text(d: &mut RaylibDrawHandle, font: &Font, text: &str, position: Vector2) {
    d.draw_text_ex(font, text, position, FONT_SIZE as f32, 0.0, Color::BLACK);
}

pub fn draw_vertex(d: &mut RaylibDrawHandle, center: Vector2, weight: &str, font: &Font) {
    d.draw_circle_lines(center.x as i32, center.y as i32, VERTEX_RADIUS, Color::BLUE);
    if !weight.is_empty() {
        let text_len = weight.chars().count();
        let x_offset: f32 = text_len as f32 * FONT_SIZE as f32 * CHAR_WIDTH;
        let y_offset: f32 = FONT_SIZE as f32 * 0.5;
        draw_text(
            d,
            font,
            weight,
            Vector2 {
                x: center.x - x_offset,
                y: center.y - y_offset,
            },
        )
    }
}

fn draw_arrowhead(d: &mut RaylibDrawHandle, position: Vector2, direction: Vector2) {
    d.draw_line_v(
        position,
        position - direction.rotated(ARROWHEAD_ANGLE) * ARROWHEAD_LEN,
        Color::BLACK,
    );
    d.draw_line_v(
        position,
        position - direction.rotated(-ARROWHEAD_ANGLE) * ARROWHEAD_LEN,
        Color::BLACK,
    );
}

pub fn draw_straight_edge(
    d: &mut RaylibDrawHandle,
    center_from: Vector2,
    center_to: Vector2,
    directional: bool,
) {
    let direction = (center_to - center_from).normalized();

    let from = center_from + direction * VERTEX_RADIUS;
    let to = center_to - direction * VERTEX_RADIUS;
    d.draw_line_v(from, to, Color::BLACK);
    if directional {
        draw_arrowhead(d, to, direction);
    }
}

pub fn draw_angled_edge(
    d: &mut RaylibDrawHandle,
    center_from: Vector2,
    center_to: Vector2,
    directional: bool,
) {
    const EDGE_BASE_ANGLE: f32 = 0.05 * PI;

    let direction = (center_to - center_from).normalized();
    let from = center_from + direction * VERTEX_RADIUS;
    let to = center_to - direction * VERTEX_RADIUS;

    let vector = to - from;
    let vector_middle = vector * 0.5;
    let mid_offset = vector_middle.length() * EDGE_BASE_ANGLE.tan();

    let midpoint = from + vector_middle + direction.rotated(0.5 * PI) * mid_offset;
    d.draw_line_v(from, midpoint, Color::BLACK);
    d.draw_line_v(midpoint, to, Color::BLACK);
    if directional {
        draw_arrowhead(d, to, (to - midpoint).normalized());
    }
}

pub fn draw_looping_edge(d: &mut RaylibDrawHandle, center: Vector2) {
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
    d.draw_line_strip(&points, Color::BLACK);

    let direction = (last_point - points[POINTS - 4]).normalized();
    d.draw_line_v(
        last_point,
        last_point - direction.rotated(ARROWHEAD_ANGLE) * ARROWHEAD_LEN,
        Color::BLACK,
    );
    d.draw_line_v(
        last_point,
        last_point - direction.rotated(-ARROWHEAD_ANGLE) * ARROWHEAD_LEN,
        Color::BLACK,
    );
}
