use gnuplot::{Caption, Color, Figure};

///
/// cargo r --bin gp
///
fn main() {
    let x = [0u32, 1, 2];
    let y = [3u32, 4, 5];
    let mut fg = Figure::new();

    fg.axes2d()
        .lines(&x, &y, &[Caption("A line"), Color("black")]);

    fg.show().expect("Failed to show");
}
