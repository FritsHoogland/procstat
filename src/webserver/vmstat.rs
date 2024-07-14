use crate::webserver::meminfo::memory_plot;
use crate::webserver::pressure::pressure_memory_plot;
use crate::{ARGS, DATA};
use crate::{
    CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE,
    LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, MESH_STYLE_FONT,
    MESH_STYLE_FONT_SIZE,
};
use chrono::{DateTime, Local};
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition, SeriesLabelPosition::UpperLeft};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Rectangle;
use plotters::prelude::*;
use plotters::style::full_palette::{
    BLUE_300, BLUE_900, LIGHTGREEN_300, LIGHTGREEN_900, ORANGE_300, ORANGE_900,
};

pub fn create_memory_alloc_plot(
    buffer: &mut [u8],
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0, start_time, end_time);
    pages_allocated_and_free(&mut multi_backend, 1, start_time, end_time)
}

pub fn create_memory_alloc_psi_plot(
    buffer: &mut [u8],
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    memory_plot(&mut multi_backend, 0, start_time, end_time);
    pages_allocated_and_free(&mut multi_backend, 1, start_time, end_time);
    pressure_memory_plot(&mut multi_backend, 2, start_time, end_time);
}

pub fn swap_inout_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.vmstat.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|v| v.timestamp)
            .min()
            .unwrap()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|v| v.timestamp)
            .max()
            .unwrap()
    };
    let latest = historical_data_read.back().unwrap();
    let high_value = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .map(|v| (v.pswpin + v.pswpout) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Swap IO", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(final_start_time..final_end_time, 0_f64..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
        .y_desc("Swap IO (pages)")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|v| (v.timestamp, v.pswpin)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:>10} {:>10} {:>10}",
            "", "min", "max", "last"
        ));

    //
    let min_total_swap = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pswpin + v.pswpout > 0_f64)
        .map(|v| v.pswpin + v.pswpout)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_swap = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pswpin + v.pswpout > 0_f64)
        .map(|v| v.pswpin + v.pswpout)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| (v.timestamp, v.pswpin + v.pswpout)),
            BLACK,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "total",
            min_total_swap,
            max_total_swap,
            (latest.pswpin + latest.pswpout)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // pgspout
    let min_pswpout = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pswpout > 0_f64)
        .map(|v| v.pswpout)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_pswpout = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pswpout > 0_f64)
        .map(|v| v.pswpout)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .filter(|v| v.pswpout > 0_f64)
                .map(|v| Circle::new((v.timestamp, v.pswpout), 4, RED.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pages swap out", min_pswpout, max_pswpout, latest.pswpout
        ))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
    // pgspin
    let min_pswpin = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pswpin > 0_f64)
        .map(|v| v.pswpin)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_pswpin = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pswpin > 0_f64)
        .map(|v| v.pswpin)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .filter(|v| v.pswpin > 0_f64)
                .map(|v| Circle::new((v.timestamp, v.pswpin), 3, GREEN.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pages swap in", min_pswpin, max_pswpin, latest.pswpin
        ))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
    // draw the legend
    contextarea
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}

pub fn pages_allocated_and_free(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.vmstat.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|v| v.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|v| v.timestamp)
            .max()
            .unwrap_or_default()
    };
    let latest = historical_data_read.back();
    let high_value_free = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .map(|v| v.pgfree * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .map(|v| {
            (v.pgalloc_dma
                + v.pgalloc_dma32
                + v.pgalloc_normal
                + v.pgalloc_device
                + v.pgalloc_movable)
                * 1.1_f64
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_fault = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .map(|v| v.pgfault_delta * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value = high_value_free.max(high_value_alloc).max(high_value_fault);

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            "Pages allocated and freed",
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(final_start_time..final_end_time, 0_f64..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
        .y_desc("Pages")
        .y_label_formatter(&|pages| {
            if pages < &1_000_f64 {
                format!("{:6.0}", pages)
            } else if pages < &1_000_000_f64 {
                format!("{:7.1} k", pages / 1_000_f64)
            } else if pages < &1_000_000_000_f64 {
                format!("{:7.1} m", pages / 1_000_000_f64)
            } else if pages < &1_000_000_000_000_f64 {
                format!("{:7.1} t", pages / 1_000_000_000_f64)
            } else {
                format!("{:7.1} p", pages / 1_000_000_000_000_f64)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|v| (v.timestamp, v.pgfree)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:>10} {:>10} {:>10}",
            "", "min", "max", "last"
        ));

    // pgfree
    let min_free = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgfree > 0_f64)
        .map(|v| v.pgfree)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_free = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgfree > 0_f64)
        .map(|v| v.pgfree)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| (v.timestamp, v.pgfree)),
            ShapeStyle {
                color: GREEN.into(),
                filled: true,
                stroke_width: 2,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgfree",
            min_free,
            max_free,
            latest.map_or(0_f64, |latest| latest.pgfree)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // pgfault (_delta)
    let min_free = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgfault_delta > 0_f64)
        .map(|v| v.pgfault_delta)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_free = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgfault_delta > 0_f64)
        .map(|v| v.pgfault_delta)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| (v.timestamp, v.pgfault_delta)),
            ShapeStyle {
                color: BLACK.into(),
                filled: true,
                stroke_width: 2,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgfault",
            min_free,
            max_free,
            latest.map_or(0_f64, |latest| latest.pgfault_delta)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // pgalloc
    let min_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| {
            (v.pgalloc_dma
                + v.pgalloc_dma32
                + v.pgalloc_normal
                + v.pgalloc_device
                + v.pgalloc_movable)
                > 0_f64
        })
        .map(|v| {
            v.pgalloc_dma
                + v.pgalloc_dma32
                + v.pgalloc_normal
                + v.pgalloc_device
                + v.pgalloc_movable
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| {
            (v.pgalloc_dma
                + v.pgalloc_dma32
                + v.pgalloc_normal
                + v.pgalloc_device
                + v.pgalloc_movable)
                > 0_f64
        })
        .map(|v| {
            v.pgalloc_dma
                + v.pgalloc_dma32
                + v.pgalloc_normal
                + v.pgalloc_device
                + v.pgalloc_movable
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| {
                    (
                        v.timestamp,
                        (v.pgalloc_dma
                            + v.pgalloc_dma32
                            + v.pgalloc_normal
                            + v.pgalloc_device
                            + v.pgalloc_movable),
                    )
                }),
            ShapeStyle {
                color: RED.into(),
                filled: true,
                stroke_width: 2,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgalloc",
            min_alloc,
            max_alloc,
            (latest.map_or(0_f64, |latest| latest.pgalloc_dma)
                + latest.map_or(0_f64, |latest| latest.pgalloc_dma32)
                + latest.map_or(0_f64, |latest| latest.pgalloc_normal)
                + latest.map_or(0_f64, |latest| latest.pgalloc_device)
                + latest.map_or(0_f64, |latest| latest.pgalloc_movable))
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    //
    // kswapd: blue
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgsteal_kswapd > 0_f64)
        .map(|v| v.pgsteal_kswapd)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgsteal_kswapd > 0_f64)
        .map(|v| v.pgsteal_kswapd)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| (v.timestamp, v.pgsteal_kswapd)),
            BLUE_900,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgsteal_kswapd",
            min_alloc,
            max_alloc,
            latest.map_or(0_f64, |latest| latest.pgsteal_kswapd)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_900.filled()));
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgscan_kswapd > 0_f64)
        .map(|v| v.pgscan_kswapd)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgscan_kswapd > 0_f64)
        .map(|v| v.pgscan_kswapd)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| (v.timestamp, v.pgscan_kswapd)),
            BLUE_300,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgscan_kswapd",
            min_alloc,
            max_alloc,
            latest.map_or(0_f64, |latest| latest.pgscan_kswapd)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_300.filled()));
    //
    // direct: orange
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgsteal_direct > 0_f64)
        .map(|v| v.pgsteal_direct)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgsteal_direct > 0_f64)
        .map(|v| v.pgsteal_direct)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .map(|vmstat| (vmstat.timestamp, vmstat.pgsteal_direct)),
            ORANGE_900,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgsteal_direct",
            min_alloc,
            max_alloc,
            latest.map_or(0_f64, |latest| latest.pgsteal_direct)
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], ORANGE_900.filled())
        });
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgscan_direct > 0_f64)
        .map(|v| v.pgscan_direct)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgscan_direct > 0_f64)
        .map(|v| v.pgscan_direct)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| (v.timestamp, v.pgscan_direct)),
            ORANGE_300,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgscan_direct",
            min_alloc,
            max_alloc,
            latest.map_or(0_f64, |latest| latest.pgscan_direct)
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], ORANGE_300.filled())
        });
    //
    // khugepaged
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgsteal_khugepaged > 0_f64)
        .map(|v| v.pgsteal_khugepaged)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgsteal_khugepaged > 0_f64)
        .map(|v| v.pgsteal_khugepaged)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| (v.timestamp, v.pgsteal_khugepaged)),
            LIGHTGREEN_900,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgsteal_khugepaged",
            min_alloc,
            max_alloc,
            latest.map_or(0_f64, |latest| latest.pgsteal_khugepaged)
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN_900.filled())
        });
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgscan_khugepaged > 0_f64)
        .map(|v| v.pgscan_khugepaged)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.pgscan_khugepaged > 0_f64)
        .map(|v| v.pgscan_khugepaged)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
                .map(|v| (v.timestamp, v.pgscan_khugepaged)),
            LIGHTGREEN_300,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pgscan_khugepaged",
            min_alloc,
            max_alloc,
            latest.map_or(0_f64, |latest| latest.pgscan_khugepaged)
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN_300.filled())
        });

    // this is a plot to only show the amount of oom kills
    let min_oom_kill = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.oom_kill > 0_f64)
        .map(|v| v.oom_kill)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_oom_kill = historical_data_read
        .iter()
        .filter(|v| v.timestamp >= final_start_time && v.timestamp <= final_end_time)
        .filter(|v| v.oom_kill > 0_f64)
        .map(|v| v.oom_kill)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|v| (v.timestamp, v.oom_kill)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "oom_kill",
            min_oom_kill,
            max_oom_kill,
            latest.map_or(0_f64, |l| l.oom_kill)
        ));
    //
    // draw the legend
    contextarea
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
