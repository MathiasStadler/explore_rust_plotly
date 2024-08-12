// FROM HERE
// https://crates.io/crates/plotly#introduction

use plotly::{Plot, Scatter};
fn main() {
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);

    plot.write_html("out.html");
}
