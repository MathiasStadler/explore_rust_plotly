#![allow(dead_code)]

// target two charts

// use std::env;
use std::path::PathBuf;

// use plotly::common::TickFormatStop;
// use plotly::layout::{Axis, RangeSelector, RangeSlider, SelectorButton, SelectorStep, StepMode};
use plotly::layout::{Axis, RangeSlider};
// use plotly::{Candlestick, Layout, Ohlc, Plot, Scatter};
use plotly::{ Layout,Plot, Scatter};
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
    sma7:f64,
    #[serde(rename = "SMA(10)")]
    sma10:f64,
    #[serde(rename = "SMA(50)")]
    sma50:f64,
    #[serde(rename = "SMA(200)")]
    sma200:f64,
    // adjusted: f64,
    // dn: f64,
    // mavg: f64,
    // up: f64,
    // direction: String,
}

fn load_apple_data() -> Vec<FinData> {
    // let mut p = PathBuf::from("./assets/");
    // /home/user/workspace_rust/explore_rust_plotly/examples/assets
    let mut p = PathBuf::from("./assets");

    // p = p.join("finance_charts_apple.csv");
    // examples/assets/output_sma_7.csv
    p = p.join("output_sma_9.csv");

    println!("Path => {}", p.display());
    // let mut p = PathBuf::from("./assets").unwrap();
    let mut rdr = csv::Reader::from_path(p).unwrap();
    let mut out = Vec::new();
    for result in rdr.deserialize() {
        let d: FinData = result.unwrap();
        out.push(d);
    }

    out
}

fn time_series_with_range_slider() {
    let data = load_apple_data();
    let date: Vec<String> = data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = data.iter().map(|d| d.high).collect();
    let sma7:Vec<f64> = data.iter().map(|d| d.sma7).collect();
    let sma10:Vec<f64> = data.iter().map(|d| d.sma10).collect();

    let trace = Scatter::new(date.clone(), high);
    let trace_sma7 = Scatter::new(date.clone(), sma7);
    let trace_sma10 = Scatter::new(date.clone(), sma10);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut plot_sma7 = Plot::new();
    plot_sma7.add_trace(trace_sma7);

    // let _plot_sma10 = Plot::new();
    // plot.add_trace(trace_sma10);

    let layout = Layout::new()
        .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
        .title("TREX Manually Set Date Range");
    plot.set_layout(layout);

    let layout_sma7 = Layout::new()
         .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
         .title("sma7");
    plot_sma7.set_layout(layout_sma7);

    // let layout_sma10 = Layout::new()
    //     .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
    //     .title("sma10");
    // plot.set_layout(layout_sma10);



    //plot.show();
    plot.write_html("out99.html");
}

fn main() {
    time_series_with_range_slider();
}

// cargo run --example f_plotly_4
