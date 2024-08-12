// Plot is a container for structs that implement the `Trace` trait. Optionally
// a `Layout` can also be specified. Its function is to serialize `Trace`s and
// the `Layout` in html format and display and/or persist the resulting plot.
//
// # Examples
//
// ```rust
use plotly::common::Mode;
use plotly::{Layout, Plot, Scatter};

fn line_and_scatter_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .name("trace1")
        .mode(Mode::Markers);
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .name("trace2")
        .mode(Mode::Lines);
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    let layout = Layout::new().title("<b>Line and Scatter Plot</b>");
    plot.set_layout(layout);

    if true {
        // We don't actually want to try and display the plot in a browser when running a doctest.
        plot.show();
    }
}

fn main() -> std::io::Result<()> {
    line_and_scatter_plot();
    Ok(())
}
