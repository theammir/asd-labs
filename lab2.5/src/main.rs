#![allow(clippy::needless_range_loop)]

use graph::{AdjMatrix, Bfs, DEFAULT_ROWS, Dfs, Search, SearchStep, VERTEX_COUNT};
use raylib::{color::Color, prelude::*};
mod draw;
mod graph;

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 600;
const WIN_MARGIN: f32 = 0.8;

const K: f32 = 1.0 - 2.0 * 0.01 - 8.0 * 0.005 - 0.15;

#[derive(Debug, Clone, Copy)]
struct VertexPos {
    v: Vector2,
    row: usize,
    col: usize,
}

fn draw_all_vertices<S: Search>(
    d: &mut RaylibDrawHandle,
    font: &Font,
    rows: &[usize],
    step: &SearchStep<S>,
) -> Vec<VertexPos> {
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
            if step.active == i {
                Color::RED
            } else if step.visited.iter().any(|(_, to)| *to == i) {
                Color::PLUM
            } else {
                Color::BLACK
            },
        );
    });

    vertex_coords
}

fn draw_all_edges<S: Search>(
    d: &mut RaylibDrawHandle,
    adj_matrix: &AdjMatrix,
    vertex_coords: &[VertexPos],
    directed: bool,
    step: &SearchStep<S>,
    hide_edges: bool,
) {
    for i in 0..vertex_coords.len() {
        let lower = if directed { 0 } else { i };
        for j in lower..vertex_coords.len() {
            let origin = vertex_coords[i];
            let destination = vertex_coords[j];
            let row_absdiff = (destination.row as i64 - origin.row as i64).abs();
            let col_absdiff = (destination.col as i64 - origin.col as i64).abs();

            if adj_matrix.0[i][j] == 1 {
                let color = if i != j && step.tree.iter().any(|(from, to)| *from == i && *to == j) {
                    Color::RED
                } else if hide_edges {
                    Color::WHITE.alpha(0.0)
                } else {
                    Color::BLACK
                };

                if i == j {
                    draw::draw_looping_edge(d, origin.v, color);
                } else if (adj_matrix.0[j][i] == 1 && directed) // symmetric
                    || (row_absdiff == 0 && col_absdiff > 1) // same row, goes through others
                    || (col_absdiff == 0 && row_absdiff > 1) // same col, goes through others
                    || (origin.v.x == destination.v.x) // same x coordinate, yes, still possible
                    || col_absdiff >= 3
                // honestly ^ whatever this is
                {
                    draw::draw_angled_edge(d, origin.v, destination.v, directed, color);
                } else {
                    draw::draw_straight_edge(d, origin.v, destination.v, directed, color);
                }
            }
        }
    }
}

fn draw_controls(d: &mut RaylibDrawHandle, font: &Font, state: KeyboardKey, hide_edges: bool) {
    draw::draw_text(
        d,
        font,
        "<F1>",
        Vector2 {
            x: 0.05 * WIN_WIDTH as f32,
            y: 0.01 * WIN_HEIGHT as f32,
        },
        if state == KeyboardKey::KEY_F1 {
            Color::RED
        } else {
            Color::BLACK
        },
    );
    draw::draw_text(
        d,
        font,
        "<F2>",
        Vector2 {
            x: 0.47 * WIN_WIDTH as f32,
            y: 0.01 * WIN_HEIGHT as f32,
        },
        if state == KeyboardKey::KEY_F2 {
            Color::RED
        } else {
            Color::BLACK
        },
    );
    draw::draw_text(
        d,
        font,
        "<F3> Hide edges",
        Vector2 {
            x: 0.68 * WIN_WIDTH as f32,
            y: 0.01 * WIN_HEIGHT as f32,
        },
        if hide_edges { Color::RED } else { Color::BLACK },
    );
    draw::draw_text(
        d,
        font,
        "<Space> Step",
        Vector2 {
            x: 0.37 * WIN_WIDTH as f32,
            y: 0.94 * WIN_HEIGHT as f32,
        },
        Color::BLACK,
    );
}

fn print_new_order<S: Search>(step: &SearchStep<S>) {
    println!("New vertex order:");
    step.visited
        .iter()
        .enumerate()
        .map(|(index, (_, to))| format!("{}->{}", to + 1, index + 1))
        .for_each(|s| print!("{} ", s));
    println!("\n");
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .log_level(TraceLogLevel::LOG_WARNING)
        .title("ASD Lab 2.4")
        .build();

    let font = rl.load_font(&thread, "FiraCode-Regular.ttf").unwrap();

    let matrix = AdjMatrix::generate(K);
    let start_vertex = matrix
        .0
        .iter()
        .position(|row| row.iter().all(|v| *v != 0))
        .unwrap_or(0);
    let mut bfs = SearchStep::new(start_vertex, VERTEX_COUNT);
    let mut dfs = SearchStep::new(start_vertex, VERTEX_COUNT);
    let mut state = KeyboardKey::KEY_F1;
    let mut hide_edges = false;

    println!("Graph:\n{}", matrix);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            state = KeyboardKey::KEY_F1;
        } else if rl.is_key_pressed(KeyboardKey::KEY_F2) {
            state = KeyboardKey::KEY_F2;
        } else if rl.is_key_pressed(KeyboardKey::KEY_F3) {
            hide_edges = !hide_edges;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            match state {
                KeyboardKey::KEY_F1 => {
                    if !matrix.search_next::<Bfs>(&mut bfs) {
                        print_new_order(&bfs);

                        let matrix: AdjMatrix = (&bfs).into();
                        println!("BFS tree:\n{}", matrix)
                    }
                }
                KeyboardKey::KEY_F2 => {
                    if !matrix.search_next::<Dfs>(&mut dfs) {
                        print_new_order(&dfs);

                        let matrix: AdjMatrix = (&dfs).into();
                        println!("DFS tree:\n{}", matrix);
                    }
                }
                _ => {}
            }
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        draw_controls(&mut d, &font, state, hide_edges);

        if state == KeyboardKey::KEY_F1 {
            let vertex_coords = draw_all_vertices(&mut d, &font, DEFAULT_ROWS, &bfs);
            draw_all_edges(&mut d, &matrix, &vertex_coords, true, &bfs, hide_edges);
        } else if state == KeyboardKey::KEY_F2 {
            let vertex_coords = draw_all_vertices(&mut d, &font, DEFAULT_ROWS, &dfs);
            draw_all_edges(&mut d, &matrix, &vertex_coords, true, &dfs, hide_edges);
        }
    }
}
