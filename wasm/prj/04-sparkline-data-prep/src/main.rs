use sparkline_data_prep::{sparkline_points_from_csv, sparkline_svg_from_csv};

fn main() {
    let csv = "120, 160, 135, 190, 175, 210";

    println!("points: {}", sparkline_points_from_csv(csv, 180.0, 48.0).unwrap());
    println!(
        "svg: {}",
        sparkline_svg_from_csv(csv, 180.0, 48.0, "#2563eb").unwrap()
    );
}
