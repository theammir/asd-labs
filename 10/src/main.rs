#![allow(clippy::needless_range_loop)]

use graph::{AdjMatrix, DEFAULT_ROWS, VERTEX_COUNT};
use raylib::{color::Color, prelude::*};
mod draw;
mod graph;

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 600;
const WIN_MARGIN: f32 = 0.8;

const K_1: f32 = 1.0 - 2.0 * 0.01 - 8.0 * 0.01 - 0.3;
const K_2: f32 = 1.0 - 2.0 * 0.005 - 8.0 * 0.005 - 0.27;

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
        draw::draw_vertex(d, vertex_coords[i].v, &((i + 1).to_string()), font);
    });

    vertex_coords
}

fn draw_all_edges(
    d: &mut RaylibDrawHandle,
    adj_matrix: &AdjMatrix,
    vertex_coords: &[VertexPos],
    directed: bool,
) {
    for i in 0..vertex_coords.len() {
        let lower = if directed { 0 } else { i };
        for j in lower..vertex_coords.len() {
            let origin = vertex_coords[i];
            let destination = vertex_coords[j];
            let row_absdiff = (destination.row as i64 - origin.row as i64).abs();
            let col_absdiff = (destination.col as i64 - origin.col as i64).abs();

            if adj_matrix.0[i][j] == 1 {
                if i == j {
                    draw::draw_looping_edge(d, origin.v);
                } else if (adj_matrix.0[j][i] == 1 && directed) // symmetric
                    || (row_absdiff == 0 && col_absdiff > 1) // same row, goes through others
                    || (col_absdiff == 0 && row_absdiff > 1) // same col, goes through others
                    || (origin.v.x == destination.v.x) // same x coordinate, yes, still possible
                    || col_absdiff >= 3
                // honestly ^ whatever this is
                {
                    draw::draw_angled_edge(d, origin.v, destination.v, directed);
                } else {
                    draw::draw_straight_edge(d, origin.v, destination.v, directed);
                }
            }
        }
    }
}

fn generate_and_print() -> (AdjMatrix, AdjMatrix, AdjMatrix, AdjMatrix) {
    let dir_matrix = AdjMatrix::generate(K_1);
    let dir_in_degrees = (0..VERTEX_COUNT).map(|i| dir_matrix.degree_in(i));
    let dir_out_degrees = (0..VERTEX_COUNT).map(|i| dir_matrix.degree_out(i));

    println!("Directed adjacency matrix:\n{}", dir_matrix);
    print!("Vertex semi-degrees (IN):  ");
    dir_in_degrees.for_each(|deg| print!("{} ", deg));
    print!("\nVertex semi-degrees (OUT): ");
    dir_out_degrees.for_each(|deg| print!("{} ", deg));
    println!("\n{:#}", dir_matrix);

    let undir_matrix = dir_matrix.undir();
    println!("\n\n\nUndirected adjacency matrix:\n{}", undir_matrix);
    println!("{:#}", undir_matrix);

    let dir_matrix2 = AdjMatrix::generate(K_2);
    let dir_paths2: Vec<_> = dir_matrix2
        .all_paths_of_2()
        .iter()
        .map(|path| (path[0] + 1, path[1] + 1, path[2] + 1))
        .map(|(i, k, j)| format!("{}->{}->{}", i, k, j))
        .collect();
    let dir_paths3: Vec<_> = dir_matrix2
        .all_paths_of_3()
        .iter()
        .map(|path| (path[0] + 1, path[1] + 1, path[2] + 1, path[3] + 1))
        .map(|(i, k, l, j)| format!("{}->{}->{}->{}", i, k, l, j))
        .collect();
    let dir_reach = dir_matrix2.reachability();
    let condensed = dir_matrix2.condensed();

    println!("\n\n\nModified adjacency matrix:\n{}", dir_matrix2);
    print!("Paths of 2: ");
    dir_paths2.iter().for_each(|path| print!("{} ", path));
    print!("\n\nPaths of 3: ");
    dir_paths3.iter().for_each(|path| print!("{} ", path));
    println!("\n\nReachability matrix:\n{}", dir_reach);
    println!("Connectivity matrix:\n{}", dir_matrix2.connectivity());
    println!(
        "Connectivity components: {:?}",
        dir_matrix2
            .conn_components()
            .iter()
            .map(|comp| comp.iter().map(|i| i + 1).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
    println!("Condensed adjacency matrix:\n{}", condensed);

    (dir_matrix, undir_matrix, dir_matrix2, condensed)
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .log_level(TraceLogLevel::LOG_WARNING)
        .title("ASD Lab 2.4")
        .build();

    let font = rl.load_font(&thread, "FiraCode-Regular.ttf").unwrap();

    let mut state: KeyboardKey = KeyboardKey::KEY_F1;

    let (dir_matrix, undir_matrix, dir_matrix2, condensed) = generate_and_print();

    while !rl.window_should_close() {
        let pressed = rl.get_key_pressed();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        if let Some(key) = pressed {
            if [
                KeyboardKey::KEY_F1,
                KeyboardKey::KEY_F2,
                KeyboardKey::KEY_F3,
                KeyboardKey::KEY_F4,
            ]
            .contains(&key)
            {
                state = key;
            }
        }

        draw::draw_text(
            &mut d,
            &font,
            "<F1>    <F2>    <F3>    <F4>",
            Vector2 {
                x: 0.2 * WIN_WIDTH as f32,
                y: 0.01 * WIN_HEIGHT as f32,
            },
        );

        match state {
            KeyboardKey::KEY_F1 => {
                let vertex_coords = draw_all_vertices(&mut d, &font, DEFAULT_ROWS);
                draw_all_edges(&mut d, &dir_matrix, &vertex_coords, true);
            }
            KeyboardKey::KEY_F2 => {
                let vertex_coords = draw_all_vertices(&mut d, &font, DEFAULT_ROWS);
                draw_all_edges(&mut d, &undir_matrix, &vertex_coords, false);
            }
            KeyboardKey::KEY_F3 => {
                let vertex_coords = draw_all_vertices(&mut d, &font, DEFAULT_ROWS);
                draw_all_edges(&mut d, &dir_matrix2, &vertex_coords, true);
            }
            KeyboardKey::KEY_F4 => {
                let vertex_coords = draw_all_vertices(&mut d, &font, &[condensed.0.len()]);
                draw_all_edges(&mut d, &condensed, &vertex_coords, true);
            }
            _ => {}
        }
    }
}
