use draw::draw_text;
use graph::{AdjMatrix, ROWS, VERTEX_COUNT};
use raylib::{color::Color, prelude::*};
mod draw;
mod graph;

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 600;
const WIN_MARGIN: f32 = 0.8;

#[derive(Clone, Copy)]
struct VertexPos {
    v: Vector2,
    row: usize,
    col: usize,
}

fn draw_all_vertices(d: &mut RaylibDrawHandle, font: &Font) -> [VertexPos; VERTEX_COUNT] {
    fn current_position(index: usize) -> (usize, usize) {
        let mut cumulative = 0;
        for (row, &count) in ROWS.iter().enumerate() {
            if index < cumulative + count {
                return (row, index - cumulative);
            }
            cumulative += count;
        }
        (usize::MAX, usize::MAX)
    }
    let winwidth = WIN_WIDTH as f32 * WIN_MARGIN;
    let winheight = WIN_HEIGHT as f32 * WIN_MARGIN;
    let vertex_coords: [VertexPos; VERTEX_COUNT] = std::array::from_fn(|i| {
        let (row, col) = current_position(i);

        let x_offset = (WIN_WIDTH as f32 - winwidth) * 0.5;
        let y_offset = (WIN_HEIGHT as f32 - winheight) * 0.5;
        VertexPos {
            v: Vector2 {
                x: (winwidth / (ROWS[row] - 1) as f32 * col as f32) + x_offset,
                y: (winheight / (ROWS.len() - 1) as f32 * row as f32) + y_offset,
            },
            row,
            col,
        }
    });

    (0..VERTEX_COUNT).for_each(|i| {
        draw::draw_vertex(d, vertex_coords[i].v, &((i + 1).to_string()), font);
    });

    vertex_coords
}

fn draw_all_edges(
    d: &mut RaylibDrawHandle,
    adj_matrix: &AdjMatrix,
    vertex_coords: &[VertexPos],
    directional: bool,
) {
    (0..VERTEX_COUNT).for_each(|i| {
        let lower = if directional { 0 } else { i };
        (lower..VERTEX_COUNT).for_each(|j| {
            let origin = vertex_coords[i];
            let destination = vertex_coords[j];
            let row_absdiff = (destination.row as i64 - origin.row as i64).abs();
            let col_absdiff = (destination.col as i64 - origin.col as i64).abs();

            if adj_matrix.0[i][j] == 1 {
                if i == j {
                    draw::draw_looping_edge(d, origin.v);
                } else if (adj_matrix.0[j][i] == 1 && directional) // symmetric
                    || (row_absdiff == 0 && col_absdiff > 1) // same row, goes through others
                    || (col_absdiff == 0 && row_absdiff > 1) // same col, goes through others
                    || (origin.v.x == destination.v.x) // same x coordinate, yes, still possible
                    || col_absdiff >= 3
                // honestly ^ whatever this is
                {
                    draw::draw_angled_edge(d, origin.v, destination.v, directional);
                } else {
                    draw::draw_straight_edge(d, origin.v, destination.v, directional);
                }
            }
        })
    })
}

fn main() {
    let dir_matrix = graph::generate_dir_matrix();
    let undir_matrix = graph::convert_to_undir(&dir_matrix);
    println!("Directional adjacency matrix:\n{}", dir_matrix);
    println!("Undirectional adjacency matrix:\n{}", undir_matrix);

    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .log_level(TraceLogLevel::LOG_WARNING)
        .title("ASD Lab 2.3")
        .build();

    let font = rl.load_font(&thread, "FiraCode-Regular.ttf").unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        draw_text(
            &mut d,
            &font,
            "<Space>",
            Vector2 {
                x: 0.8 * WIN_WIDTH as f32,
                y: 0.95 * WIN_HEIGHT as f32,
            },
        );

        let vertex_coords = draw_all_vertices(&mut d, &font);
        if d.is_key_down(KeyboardKey::KEY_SPACE) {
            draw_all_edges(&mut d, &dir_matrix, &vertex_coords, true);
        } else {
            draw_all_edges(&mut d, &undir_matrix, &vertex_coords, false);
        }
    }
}
