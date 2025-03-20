#![allow(clippy::needless_range_loop)]

use std::collections::VecDeque;

use graph::{DEFAULT_ROWS, Graph, KruskalStep, VERTEX_COUNT};
use raylib::{color::Color, prelude::*};
mod draw;
mod graph;

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 600;
const WIN_MARGIN: f32 = 0.8;
const OVERLAY_FONT_SIZE: i32 = 24;

const K: f32 = 1.0 - 2.0 * 0.01 - 8.0 * 0.005 - 0.15;

#[derive(Debug, Clone, Copy)]
struct VertexPos {
    v: Vector2,
    row: usize,
    col: usize,
}

fn draw_all_vertices(d: &mut RaylibDrawHandle, font: &Font, rows: &[usize]) -> Vec<VertexPos> {
    fn current_position(index: usize, rows: &[usize]) -> (usize, usize) {
        let mut cumulative = 0;
        for (row, &count) in rows.iter().enumerate() {
            if index < cumulative + count {
                return (row, index - cumulative);
            }
            cumulative += count;
        }
        (usize::MAX, usize::MAX)
    }
    let winwidth = WIN_WIDTH as f32 * WIN_MARGIN;
    let winheight = WIN_HEIGHT as f32 * WIN_MARGIN;
    let vertex_count = rows.iter().sum();
    let vertex_coords: Vec<VertexPos> = Vec::from_iter((0..vertex_count).map(|i| {
        let (row, col) = current_position(i, rows);

        let x_offset = (WIN_WIDTH as f32 - winwidth) * 0.5;
        let y_offset = (WIN_HEIGHT as f32 - winheight) * 0.5;
        VertexPos {
            v: Vector2 {
                x: (winwidth / (DEFAULT_ROWS[row] - 1) as f32 * col as f32) + x_offset,
                y: (winheight / (DEFAULT_ROWS.len() - 1) as f32 * row as f32) + y_offset,
            },
            row,
            col,
        }
    }));

    (0..vertex_count).for_each(|i| {
        draw::draw_vertex(
            d,
            vertex_coords[i].v,
            &((i + 1).to_string()),
            font,
            Color::BLACK,
        );
    });

    vertex_coords
}

fn draw_all_edges(
    d: &mut RaylibDrawHandle,
    graph: &Graph,
    vertex_coords: &[VertexPos],
    font: &Font,
    directed: bool,
    hide_edges: bool,
    step: &KruskalStep,
) {
    for i in 0..vertex_coords.len() {
        let lower = if directed { 0 } else { i };
        for j in lower..vertex_coords.len() {
            let origin = vertex_coords[i];
            let destination = vertex_coords[j];
            let row_absdiff = (destination.row as i64 - origin.row as i64).abs();
            let col_absdiff = (destination.col as i64 - origin.col as i64).abs();

            if let Some(weight) = graph.0[i][j] {
                let step_current = step.current.unwrap_or((usize::MAX, usize::MAX, 0));
                let color = if step_current.0 == i && step_current.1 == j {
                    Color::BLUE
                } else if step
                    .tree
                    .iter()
                    .any(|(row, column, _)| *row == i && *column == j)
                {
                    Color::RED
                } else if hide_edges {
                    Color::WHITE.alpha(0.0)
                } else {
                    Color::BLACK
                };
                let mut angled = false;

                if i == j {
                    draw::draw_looping_edge(d, origin.v, color);
                } else if (graph.0[j][i].is_some() && directed) // symmetric
                    || (row_absdiff == 0 && col_absdiff > 1) // same row, goes through others
                    || (col_absdiff == 0 && row_absdiff > 1) // same col, goes through others
                    || (origin.v.x == destination.v.x) // same x coordinate, yes, still possible
                    || col_absdiff >= 3
                // honestly ^ whatever this is
                {
                    draw::draw_angled_edge(d, origin.v, destination.v, directed, color);
                    angled = true;
                } else {
                    draw::draw_straight_edge(d, origin.v, destination.v, directed, color);
                }

                if i < j {
                    draw::draw_edge_weight(
                        d,
                        origin.v,
                        destination.v,
                        &weight.to_string(),
                        font,
                        color,
                        angled,
                    );
                }
            }
        }
    }
}

fn draw_controls(d: &mut RaylibDrawHandle, font: &Font, hide_edges: bool) {
    draw::draw_text(
        d,
        font,
        "<F3> Hide edges",
        Vector2 {
            x: 0.12 * WIN_WIDTH as f32,
            y: 0.01 * WIN_HEIGHT as f32,
        },
        OVERLAY_FONT_SIZE as f32,
        if hide_edges { Color::RED } else { Color::BLACK },
    );
    draw::draw_text(
        d,
        font,
        "<Space> Step",
        Vector2 {
            x: 0.1 * WIN_WIDTH as f32,
            y: 0.96 * WIN_HEIGHT as f32,
        },
        OVERLAY_FONT_SIZE as f32,
        Color::BLACK,
    );
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .log_level(TraceLogLevel::LOG_WARNING)
        .title("ASD Lab 2.6")
        .build();

    let font = rl.load_font(&thread, "FiraCode-Regular.ttf").unwrap();

    let matrix = Graph::generate(K);
    let mut edges: Vec<(usize, usize, u32)> = matrix
        .0
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &cell)| cell.map(|w| (i, j, w)))
        })
        .filter(|(i, j, _)| *i < *j)
        .collect();
    edges.sort_by_key(|(_, _, w)| *w);

    let mut edges_deque = VecDeque::from(edges);
    let mut step = KruskalStep::new(VERTEX_COUNT);

    let mut hide_edges = false;

    println!("Graph:\n{}", matrix);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            Graph::kruskal_step(&mut edges_deque, &mut step);
        } else if rl.is_key_pressed(KeyboardKey::KEY_F3) {
            hide_edges = !hide_edges;
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        let vertex_pos = draw_all_vertices(&mut d, &font, DEFAULT_ROWS);
        draw_all_edges(
            &mut d,
            &matrix,
            &vertex_pos,
            &font,
            false,
            hide_edges,
            &step,
        );

        draw_controls(&mut d, &font, hide_edges);
        draw::draw_text(
            &mut d,
            &font,
            &(String::from("Weight sum: ") + &step.weight_sum().to_string()),
            Vector2 {
                x: 0.8 * WIN_WIDTH as f32,
                y: 0.01 * WIN_HEIGHT as f32,
            },
            OVERLAY_FONT_SIZE as f32,
            Color::BLUE,
        );
    }
}
