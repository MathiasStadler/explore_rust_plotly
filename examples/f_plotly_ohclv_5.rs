#![allow(dead_code)]

// target two charts

use std::path::PathBuf;
use plotly::layout::{Axis, RangeSlider};
use plotly::{Layout, Plot, Scatter};
use serde::Deserialize;

// date,open,high,low,close,volume,sma7
// Date,Open,High,Low,Close,Volume,SMA(7),SMA(10),SMA(50),SMA(200)
#[derive(Deserialize)]
#[allow(dead_code)]
struct FinData {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Open")]
    open: f64,
    #[serde(rename = "High")]
    high: f64,
    #[serde(rename = "Low")]
    low: f64,
    #[serde(rename = "Close")]
    close: f64,
    #[serde(rename = "Volume")]
    volume: f64,
    #[serde(rename = "SMA(7)")]
    sma7: f64,
    #[serde(rename = "SMA(10)")]
    sma10: f64,
    #[serde(rename = "SMA(50)")]
    sma50: f64,
    #[serde(rename = "SMA(200)")]
    sma200: f64,
}

fn load_data_csv() -> Vec<FinData> {
    let mut p = PathBuf::from("./assets");

    p = p.join("output_sma_9.csv");

    println!("Path => {}", p.display());

    let mut rdr = csv::Reader::from_path(p).unwrap();
    let mut out = Vec::new();
    for result in rdr.deserialize() {
        let d: FinData = result.unwrap();
        out.push(d);
    }

    out
}

fn time_series_with_range_slider() {
    let data = load_data_csv();
    let date: Vec<String> = data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date.clone(), high);

    trace.

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
        .title("TREX Manually Set Date Range");
    plot.set_layout(layout);

    

    plot.write_html("out11.html");
    
}

fn main() {
    time_series_with_range_slider();
    
}

// cargo run --example f_plotly_5
