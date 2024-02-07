use crate::webserver::meminfo::memory_plot;
use crate::webserver::pressure::pressure_memory_plot;
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};
use crate::{ARGS, HISTORY};
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition, SeriesLabelPosition::UpperLeft};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Rectangle;
use plotters::prelude::*;
use plotters::style::full_palette::{BLUE_300, BLUE_900, ORANGE_900, ORANGE_300, LIGHTGREEN_900, LIGHTGREEN_300};

pub fn create_memory_alloc_plot(
    buffer: &mut [u8],
) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    pages_allocated_and_free(&mut multi_backend, 1)
}

pub fn create_memory_alloc_psi_plot(
    buffer: &mut [u8],
) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    memory_plot(&mut multi_backend, 0);
    pages_allocated_and_free(&mut multi_backend, 1);
    pressure_memory_plot(&mut multi_backend, 2);
}

pub fn swap_inout_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
) {
    let historical_data_read = HISTORY.vmstat.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|vmstat| vmstat.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|vmstat| vmstat.timestamp)
        .max()
        .unwrap();
    let latest = historical_data_read
        .back()
        .unwrap();
    let high_value = historical_data_read
        .iter()
        .map(|vmstat| (vmstat.pswpin + vmstat.pswpout) * 1.1_f64 )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Swap IO", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Swap IO (pages)")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|vmstat| (vmstat.timestamp, vmstat.pswpin)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
     
    //
    let min_total_swap = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpin + vmstat.pswpout > 0_f64)
        .map(|vmstat| vmstat.pswpin + vmstat.pswpout)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_swap = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpin + vmstat.pswpout > 0_f64)
        .map(|vmstat| vmstat.pswpin + vmstat.pswpout)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .map(|vmstat| (vmstat.timestamp, vmstat.pswpin + vmstat.pswpout)), BLACK))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "total", min_total_swap, max_total_swap, (latest.pswpin + latest.pswpout)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // pgspout
    let min_pswpout = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpout > 0_f64)
        .map(|vmstat| vmstat.pswpout)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_pswpout = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpout > 0_f64)
        .map(|vmstat| vmstat.pswpout)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|vmstat| vmstat.pswpout > 0_f64)
                                                .map(|vmstat| Circle::new((vmstat.timestamp, vmstat.pswpout), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pages swap out", min_pswpout, max_pswpout, latest.pswpout))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
    // pgspin
    let min_pswpin = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpin > 0_f64)
        .map(|vmstat| vmstat.pswpin)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_pswpin = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpin > 0_f64)
        .map(|vmstat| vmstat.pswpin)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|vmstat| vmstat.pswpin > 0_f64)
                                                .map(|vmstat| Circle::new((vmstat.timestamp, vmstat.pswpin), 3, GREEN.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pages swap in", min_pswpin, max_pswpin, latest.pswpin))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
    // draw the legend
    contextarea.configure_series_labels()
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
) {
    let historical_data_read = HISTORY.vmstat.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|vmstat| vmstat.timestamp)
        .min()
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .map(|vmstat| vmstat.timestamp)
        .max()
        .unwrap_or_default();
    let latest = historical_data_read
        .back();
    let high_value_free = historical_data_read
        .iter()
        .map(|vmstat| vmstat.pgfree * 1.1_f64 )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_alloc = historical_data_read
        .iter()
        .map(|vmstat| (vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable) * 1.1_f64 )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_fault = historical_data_read
        .iter()
        .map(|vmstat| vmstat.pgfault_delta * 1.1_f64 )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value = high_value_free.max(high_value_alloc).max(high_value_fault);

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Pages allocated and freed", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Pages")
        .y_label_formatter(&|pages| {
                 if pages < &1_000_f64             { format!("{:6.0}",   pages)                       }
            else if pages < &1_000_000_f64         { format!("{:7.1} k", pages/1_000_f64)             }
            else if pages < &1_000_000_000_f64     { format!("{:7.1} m", pages/1_000_000_f64)         }
            else if pages < &1_000_000_000_000_f64 { format!("{:7.1} t", pages/1_000_000_000_f64)     }
            else                                   { format!("{:7.1} p", pages/1_000_000_000_000_f64) } })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|vmstat| (vmstat.timestamp, vmstat.pgfree)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
     
    // pgfree
    let min_free = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgfree > 0_f64)
        .map(|vmstat| vmstat.pgfree)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_free = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgfree > 0_f64)
        .map(|vmstat| vmstat.pgfree)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgfree)), ShapeStyle { color: GREEN.into(), filled: true, stroke_width: 2 }))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgfree", min_free, max_free, latest.map_or(0_f64, |latest| latest.pgfree)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // pgfault (_delta)
    let min_free = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgfault_delta > 0_f64)
        .map(|vmstat| vmstat.pgfault_delta)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_free = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgfault_delta > 0_f64)
        .map(|vmstat| vmstat.pgfault_delta)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgfault_delta)), ShapeStyle { color: BLACK.into(), filled: true, stroke_width: 2 }))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgfault", min_free, max_free, latest.map_or(0_f64, |latest| latest.pgfault_delta)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // pgalloc
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| (vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable) > 0_f64)
        .map(|vmstat| vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| (vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable) > 0_f64)
        .map(|vmstat| vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, (vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable))), ShapeStyle { color: RED.into(), filled: true, stroke_width: 2 }))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgalloc", min_alloc, max_alloc, (latest.map_or(0_f64, |latest| latest.pgalloc_dma) + latest.map_or(0_f64, |latest| latest.pgalloc_dma32) + latest.map_or(0_f64, |latest| latest.pgalloc_normal) + latest.map_or(0_f64, |latest| latest.pgalloc_device) + latest.map_or(0_f64, |latest| latest.pgalloc_movable))))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // 
    // kswapd: blue
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_kswapd > 0_f64)
        .map(|vmstat| vmstat.pgsteal_kswapd)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_kswapd > 0_f64)
        .map(|vmstat| vmstat.pgsteal_kswapd)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .map(|vmstat| (vmstat.timestamp, vmstat.pgsteal_kswapd)), BLUE_900))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgsteal_kswapd", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgsteal_kswapd)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_900.filled()));
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_kswapd > 0_f64)
        .map(|vmstat| vmstat.pgscan_kswapd)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_kswapd > 0_f64)
        .map(|vmstat| vmstat.pgscan_kswapd)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgscan_kswapd)), BLUE_300))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgscan_kswapd", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgscan_kswapd)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_300.filled()));
    //
    // direct: orange
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_direct > 0_f64)
        .map(|vmstat| vmstat.pgsteal_direct)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_direct > 0_f64)
        .map(|vmstat| vmstat.pgsteal_direct)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgsteal_direct)), ORANGE_900))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgsteal_direct", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgsteal_direct)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], ORANGE_900.filled()));
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_direct > 0_f64)
        .map(|vmstat| vmstat.pgscan_direct)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_direct > 0_f64)
        .map(|vmstat| vmstat.pgscan_direct)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgscan_direct)), ORANGE_300))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgscan_direct", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgscan_direct)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], ORANGE_300.filled()));
    //
    // khugepaged
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_khugepaged > 0_f64)
        .map(|vmstat| vmstat.pgsteal_khugepaged)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_khugepaged > 0_f64)
        .map(|vmstat| vmstat.pgsteal_khugepaged)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgsteal_khugepaged)), LIGHTGREEN_900))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgsteal_khugepaged", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgsteal_khugepaged)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN_900.filled()));
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_khugepaged > 0_f64)
        .map(|vmstat| vmstat.pgscan_khugepaged)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_khugepaged > 0_f64)
        .map(|vmstat| vmstat.pgscan_khugepaged)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgscan_khugepaged)), LIGHTGREEN_300))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgscan_khugepaged", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgscan_khugepaged)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN_300.filled()));
    //
    // draw the legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
