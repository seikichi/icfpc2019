use lib::task::{Task, Point};
use std::cmp::{min,max};
use std::io::{self, Read};

struct Bounds {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

fn set_canvas(scale: i32, bounds: &Bounds) {
    println!(r#"<g transform="translate(0, {})">"#, (bounds.top as f32 + 0.5) * (scale as f32));
    println!(r#"<g transform="scale({}, -{})">"#, scale, scale);
}

fn draw_rect(x: i32, y: i32, width: i32, height: i32, stroke: &str, fill: &str, stroke_width: i32) {
    println!(r#"<rect x="{}" y="{}" width="{}" height="{}" stroke="{}" fill="{}" stroke-width="{}" />"#,
        x, y, width, height, stroke, fill, stroke_width);
}

fn get_bounds(task: &Task) -> Bounds {
    let mut left = std::i32::MAX;
    let mut right = std::i32::MIN;
    let mut top = std::i32::MIN;
    let mut bottom = std::i32::MAX;

    let map: &Vec<Point> = &task.map.0;
    for point in map {
        left = min(left, point.x);
        right = max(right, point.x);
        top = max(top, point.y);
        bottom = min(bottom, point.y);
    }

    Bounds{left, right, top, bottom}
}

fn draw_bounding_rect(b: &Bounds) {
    draw_rect(b.left, b.bottom, b.right - b.left, b.top - b.bottom, "black", "#eee", 0);
}

fn draw_cell(x: i32, y: i32, color: &str) {
    draw_rect(x, y, 1, 1, "black", color, 0);
}

fn draw_text(x: f32, y: f32, size: i32, text: &str) {
    println!(r#"<text x="{}" y="{}" font-size="{}">{}</text>"#, x, y, size, text);
}

fn draw_obstacles(task: &Task) {
    for obstacle in &task.obstacles {
        let map = &obstacle.0;
        for point in map {
            draw_cell(point.x, point.y, "#666");
        }
    }
}

fn draw_boosters(task: &Task) {
    for booster in &task.boosters {
        let point = &booster.point;
        let code = &booster.code;
        draw_cell(point.x, point.y, "#cc7");
        draw_text(point.x as f32 + 0.1, point.y as f32+0.8, 1, code.symbol());
    }
}

fn visualize(s: &str) {
    let task = Task::from(s);

    let bounds = get_bounds(&task);

    let dot_per_unit = 8;
    let screen_width = (bounds.right - bounds.left + 1) * dot_per_unit;
    let screen_height = (bounds.top - bounds.bottom + 1) * dot_per_unit;

    println!(r#"<?xml version="1.0"?><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">"#, screen_width, screen_height);

    set_canvas(dot_per_unit, &bounds);
    draw_bounding_rect(&bounds);
    draw_obstacles(&task);
    draw_boosters(&task);

    println!(r#"</g>"#);
    println!(r#"</g>"#);
    println!(r#"</svg>"#);
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    visualize(&buffer.trim());
    Ok(())
}
