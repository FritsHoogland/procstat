use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::coord::Shift;
use plotters::prelude::full_palette::LIGHTGREEN;
use plotters::prelude::*;
use plotters::prelude::{
    AreaSeries, LineSeries, Palette99, ShapeStyle, BLACK, RED, TRANSPARENT, WHITE,
};
use plotters::style::full_palette::{
    AMBER_100, AMBER_400, AMBER_700, BROWN_400, BROWN_500, GREY, GREY_300, GREY_600, GREY_900,
    LIGHTGREEN_400, ORANGE, PURPLE_100, PURPLE_800, RED_100,
};

use crate::webserver::pressure::pressure_memory_plot;
use crate::webserver::vmstat::swap_inout_plot;
use crate::ARGS;
use crate::{
    CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABELS_STYLE_FONT,
    LABELS_STYLE_FONT_SIZE, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT,
    MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE,
};
use sysctl::{Ctl, Sysctl};

pub fn create_memory_plot(buffer: &mut [u8]) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((1, 1));
    memory_plot(&mut multi_backend, 0);
}

pub fn create_memory_psi_plot(buffer: &mut [u8]) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    pressure_memory_plot(&mut multi_backend, 1);
}

pub fn create_memory_commit(buffer: &mut [u8]) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    committed_mem_plot(&mut multi_backend, 1);
}

pub fn create_memory_active_inactive_plot(buffer: &mut [u8]) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    active_inactive_mem_plot(&mut multi_backend, 1);
}

pub fn create_memory_swap_plot(buffer: &mut [u8]) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    swap_space_plot(&mut multi_backend, 1);
}

pub fn create_memory_swap_inout_plot(buffer: &mut [u8]) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    memory_plot(&mut multi_backend, 0);
    swap_inout_plot(&mut multi_backend, 1);
    swap_space_plot(&mut multi_backend, 2);
}
pub fn memory_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
) {
    let historical_data_read = HISTORY.memory.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|meminfo| meminfo.timestamp)
        .min()
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .map(|meminfo| meminfo.timestamp)
        .max()
        .unwrap_or_default();
    let low_value: f64 = 0.0;
    let high_value = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.memtotal * 1.1_f64) / 1024_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read.back();
    let min_free_kbytes: f64 = match Ctl::new("vm.min_free_kbytes") {
        Ok(value) => value
            .description()
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap_or_default(),
        Err(_) => 0_f64,
    };

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Memory", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_label_formatter(&|size| {
            if size < &1024_f64 {
                format!("{:5.0} MB", size)
            } else if size < &10240_f64 {
                format!("{:5.1} GB", size / 1024_f64)
            } else {
                format!("{:5.0} GB", size / 1024_f64)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|meminfo| (meminfo.timestamp, meminfo.memtotal)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:>10} {:>10} {:>10}",
            "", "min MB", "max MB", "last MB"
        ));
    //
    // memory total; this is the total limit, so it doesn't need to be stacked.
    let min_memory_total = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memtotal)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_memory_total = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memtotal)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.memtotal / 1024_f64)),
            0.0,
            Palette99::pick(1),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "memory total",
            min_memory_total / 1024_f64,
            max_memory_total / 1024_f64,
            latest.map_or(0_f64, |latest| latest.memtotal / 1024_f64)
        ))
        .legend(move |(x, y)| {
            Rectangle::new(
                [(x - 3, y - 3), (x + 3, y + 3)],
                Palette99::pick(1).filled(),
            )
        });
    //
    // The next memory areas should be stacked, so the 'top' memory allocation should be shown first, then one allocation removed, etc.
    // This is a manually constructed stacked area graph.
    // First hugepages used.
    // hugepages_used, hugepages_reserved, hugepages_free (=hugepages_total), buffers, swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, cached, mapped, anonymous, memfree
    let min_hugepages_used = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.hugepages_total - meminfo.hugepages_free) * meminfo.hugepagesize)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_hugepages_used = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.hugepages_total - meminfo.hugepages_free) * meminfo.hugepagesize)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    // issue: https://github.com/FritsHoogland/procstat/issues/1: hugepages_used should be
    // hugepages_total, so that hugepages_free can overwrite all that is free.
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    ((meminfo.hugepages_total * meminfo.hugepagesize)
                        + meminfo.memfree
                        + meminfo.cached
                        + meminfo.anonpages
                        + meminfo.slab
                        + meminfo.buffers
                        + meminfo.pagetables
                        + meminfo.hardwarecorrupted
                        + meminfo.kernelstack)
                        / 1024_f64,
                )
            }),
            0.0,
            GREY_900,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "hugepages used",
            min_hugepages_used / 1024_f64,
            max_hugepages_used / 1024_f64,
            ((latest.map_or(0_f64, |latest| latest.hugepages_total)
                - latest.map_or(0_f64, |latest| latest.hugepages_free))
                * latest.map_or(0_f64, |latest| latest.hugepagesize))
                / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREY_900.filled()));
    // hugepages_reserved
    // reserved hugepages are hugepages that are still counted as free
    let min_hugepages_reserved = historical_data_read
        .iter()
        .map(|meminfo| meminfo.hugepages_reserved * meminfo.hugepagesize)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_hugepages_reserved = historical_data_read
        .iter()
        .map(|meminfo| meminfo.hugepages_reserved * meminfo.hugepagesize)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    ((meminfo.hugepages_free * meminfo.hugepagesize)
                        + meminfo.memfree
                        + meminfo.cached
                        + meminfo.anonpages
                        + meminfo.slab
                        + meminfo.buffers
                        + meminfo.pagetables
                        + meminfo.hardwarecorrupted
                        + meminfo.kernelstack)
                        / 1024_f64,
                )
            }),
            0.0,
            GREY_600,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "hugepages reserved",
            min_hugepages_reserved / 1024_f64,
            max_hugepages_reserved / 1024_f64,
            (latest.map_or(0_f64, |latest| latest.hugepages_reserved)
                * latest.map_or(0_f64, |latest| latest.hugepagesize))
                / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREY_600.filled()));
    // hugepages_free
    // actual free hugepages are hugepages_free - hugepages_reserved
    // if no hugepages are reserved, the free hugepages here will overwrite the above
    // hugepages_reserved drawing.
    let min_hugepages_free = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.hugepages_free - meminfo.hugepages_reserved) * meminfo.hugepagesize)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_hugepages_free = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.hugepages_free - meminfo.hugepages_reserved) * meminfo.hugepagesize)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (((meminfo.hugepages_free - meminfo.hugepages_reserved)
                        * meminfo.hugepagesize)
                        + meminfo.memfree
                        + meminfo.cached
                        + meminfo.anonpages
                        + meminfo.slab
                        + meminfo.buffers
                        + meminfo.pagetables
                        + meminfo.hardwarecorrupted
                        + meminfo.kernelstack)
                        / 1024_f64,
                )
            }),
            0.0,
            GREY_300,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "hugepages free",
            min_hugepages_free / 1024_f64,
            max_hugepages_free / 1024_f64,
            ((latest.map_or(0_f64, |latest| latest.hugepages_free)
                - latest.map_or(0_f64, |latest| latest.hugepages_reserved))
                * latest.map_or(0_f64, |latest| latest.hugepagesize))
                / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREY_300.filled()));

    // kernelstack
    let min_kernelstack = historical_data_read
        .iter()
        .map(|meminfo| meminfo.kernelstack)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_kernelstack = historical_data_read
        .iter()
        .map(|meminfo| meminfo.kernelstack)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.cached
                        + meminfo.anonpages
                        + meminfo.slab
                        + meminfo.buffers
                        + meminfo.pagetables
                        + meminfo.hardwarecorrupted
                        + meminfo.kernelstack)
                        / 1024_f64,
                )
            }),
            0.0,
            ORANGE,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "kernelstack",
            min_kernelstack / 1024_f64,
            max_kernelstack / 1024_f64,
            latest.map_or(0_f64, |latest| latest.kernelstack) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], ORANGE.filled()));
    // swapcached
    let min_swapcached = historical_data_read
        .iter()
        .map(|meminfo| meminfo.swapcached)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_swapcached = historical_data_read
        .iter()
        .map(|meminfo| meminfo.swapcached)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.cached
                        + meminfo.anonpages
                        + meminfo.slab
                        + meminfo.buffers
                        + meminfo.pagetables
                        + meminfo.hardwarecorrupted
                        + meminfo.swapcached)
                        / 1024_f64,
                )
            }),
            0.0,
            BLUE,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "swapcached",
            min_swapcached / 1024_f64,
            max_swapcached / 1024_f64,
            latest.map_or(0_f64, |latest| latest.swapcached) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE.filled()));
    // hardwarecorrupted
    let min_hardwarecorrupted = historical_data_read
        .iter()
        .map(|meminfo| meminfo.hardwarecorrupted)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_hardwarecorrupted = historical_data_read
        .iter()
        .map(|meminfo| meminfo.hardwarecorrupted)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.cached
                        + meminfo.anonpages
                        + meminfo.slab
                        + meminfo.buffers
                        + meminfo.pagetables
                        + meminfo.hardwarecorrupted)
                        / 1024_f64,
                )
            }),
            0.0,
            RED_100,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "hardwarecorrupted",
            min_hardwarecorrupted / 1024_f64,
            max_hardwarecorrupted / 1024_f64,
            latest.map_or(0_f64, |latest| latest.hardwarecorrupted) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED_100.filled()));
    // pagetables
    let min_pagetables = historical_data_read
        .iter()
        .map(|meminfo| meminfo.pagetables)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_pagetables = historical_data_read
        .iter()
        .map(|meminfo| meminfo.pagetables)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.cached
                        + meminfo.anonpages
                        + meminfo.slab
                        + meminfo.buffers
                        + meminfo.pagetables)
                        / 1024_f64,
                )
            }),
            0.0,
            LIGHTGREEN_400,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "pagetables",
            min_pagetables / 1024_f64,
            max_pagetables / 1024_f64,
            latest.map_or(0_f64, |latest| latest.pagetables) / 1024_f64
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN_400.filled())
        });
    // slab
    let min_slab = historical_data_read
        .iter()
        .map(|meminfo| meminfo.slab)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_slab = historical_data_read
        .iter()
        .map(|meminfo| meminfo.slab)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.cached
                        + meminfo.anonpages
                        + meminfo.buffers
                        + meminfo.slab)
                        / 1024_f64,
                )
            }),
            0.0,
            PURPLE_800,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "slab",
            min_slab / 1024_f64,
            max_slab / 1024_f64,
            latest.map_or(0_f64, |latest| latest.slab) / 1024_f64
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE_800.filled())
        });
    // buffers
    let min_buffers = historical_data_read
        .iter()
        .map(|meminfo| meminfo.buffers)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_buffers = historical_data_read
        .iter()
        .map(|meminfo| meminfo.buffers)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree + meminfo.cached + meminfo.anonpages + meminfo.buffers)
                        / 1024_f64,
                )
            }),
            0.0,
            CYAN,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "buffers",
            min_buffers / 1024_f64,
            max_buffers / 1024_f64,
            latest.map_or(0_f64, |latest| latest.buffers) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], CYAN.filled()));
    // anonymous
    let min_anonpages = historical_data_read
        .iter()
        .map(|meminfo| meminfo.anonpages)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_anonpages = historical_data_read
        .iter()
        .map(|meminfo| meminfo.anonpages)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree + meminfo.cached + meminfo.anonpages) / 1024_f64,
                )
            }),
            0.0,
            BROWN_500,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "anonpages",
            min_anonpages / 1024_f64,
            max_anonpages / 1024_f64,
            latest.map_or(0_f64, |latest| latest.anonpages) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BROWN_500.filled()));
    // dirty sits inside 'cached'
    // dirty
    let min_dirty = historical_data_read
        .iter()
        .map(|meminfo| meminfo.dirty)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_dirty = historical_data_read
        .iter()
        .map(|meminfo| meminfo.dirty)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree + meminfo.cached) / 1024_f64,
                )
            }),
            0.0,
            RED,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "dirty",
            min_dirty / 1024_f64,
            max_dirty / 1024_f64,
            latest.map_or(0_f64, |latest| latest.dirty) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // shmem
    let min_shmem = historical_data_read
        .iter()
        .map(|meminfo| meminfo.shmem)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_shmem = historical_data_read
        .iter()
        .map(|meminfo| meminfo.shmem)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree + (meminfo.cached - meminfo.dirty)) / 1024_f64,
                )
            }),
            0.0,
            PURPLE_100,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "shmem (smallpages)",
            min_shmem / 1024_f64,
            max_shmem / 1024_f64,
            latest.map_or(0_f64, |latest| latest.shmem) / 1024_f64
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE_100.filled())
        });
    // mapped
    let min_mapped = historical_data_read
        .iter()
        .map(|meminfo| meminfo.mapped)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_mapped = historical_data_read
        .iter()
        .map(|meminfo| meminfo.mapped)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree + (meminfo.cached - meminfo.dirty - meminfo.shmem)) / 1024_f64,
                )
            }),
            0.0,
            AMBER_400,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "mapped",
            min_mapped / 1024_f64,
            max_mapped / 1024_f64,
            latest.map_or(0_f64, |latest| latest.mapped) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], AMBER_400.filled()));
    // cached
    let min_cached = historical_data_read
        .iter()
        .map(|meminfo| meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_cached = historical_data_read
        .iter()
        .map(|meminfo| meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + (meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty))
                        / 1024_f64,
                )
            }),
            0.0,
            AMBER_100,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "cached",
            min_cached / 1024_f64,
            max_cached / 1024_f64,
            (latest.map_or(0_f64, |latest| latest.cached)
                - latest
                    .map_or(0_f64, |latest| latest.mapped)
                    .max(latest.map_or(0_f64, |latest| latest.shmem))
                - latest.map_or(0_f64, |latest| latest.dirty))
                / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], AMBER_100.filled()));
    // memfree
    let min_memfree = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memfree)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_memfree = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memfree)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.memfree / 1024_f64)),
            0.0,
            LIGHTGREEN,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "memfree",
            min_memfree / 1024_f64,
            max_memfree / 1024_f64,
            latest.map_or(0_f64, |latest| latest.memfree) / 1024_f64
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN.filled())
        });
    // special drawing
    // memavailable
    let min_memavailable = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memavailable)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_memavailable = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memavailable)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.memavailable / 1024_f64)),
            ShapeStyle {
                color: RED.into(),
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "memavailable",
            min_memavailable / 1024_f64,
            max_memavailable / 1024_f64,
            latest.map_or(0_f64, |latest| latest.memavailable) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    if min_free_kbytes != 0_f64 {
        // min_free_kbytes / pages_min
        contextarea
            .draw_series(LineSeries::new(
                historical_data_read
                    .iter()
                    .map(|meminfo| (meminfo.timestamp, min_free_kbytes / 1024_f64)),
                ShapeStyle {
                    color: BLACK.into(),
                    filled: false,
                    stroke_width: 1,
                },
            ))
            .unwrap();
        // pages_low
        contextarea
            .draw_series(LineSeries::new(
                historical_data_read.iter().map(|meminfo| {
                    (
                        meminfo.timestamp,
                        (min_free_kbytes + (min_free_kbytes / 4_f64)) / 1024_f64,
                    )
                }),
                ShapeStyle {
                    color: BLACK.into(),
                    filled: false,
                    stroke_width: 1,
                },
            ))
            .unwrap();
        // pages_high
        contextarea
            .draw_series(LineSeries::new(
                historical_data_read.iter().map(|meminfo| {
                    (
                        meminfo.timestamp,
                        (min_free_kbytes + (min_free_kbytes / 2_f64)) / 1024_f64,
                    )
                }),
                ShapeStyle {
                    color: BLACK.into(),
                    filled: false,
                    stroke_width: 1,
                },
            ))
            .unwrap();
    }
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

fn swap_space_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
) {
    let historical_data_read = HISTORY.memory.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|meminfo| meminfo.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|meminfo| meminfo.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.swaptotal * 1.1_f64) / 1024_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let latest = historical_data_read.back().unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Swap usage", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_label_formatter(&|size| {
            if size < &1024_f64 {
                format!("{:5.0} MB", size)
            } else if size < &10240_f64 {
                format!("{:5.1} GB", size / 1024_f64)
            } else {
                format!("{:5.0} GB", size / 1024_f64)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|meminfo| (meminfo.timestamp, meminfo.swaptotal)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:>10} {:>10} {:>10}",
            "", "min MB", "max MB", "last MB"
        ));
    // swap total; this is the total, so it doesn't need to be stacked.
    let min_swap_total = historical_data_read
        .iter()
        .map(|meminfo| meminfo.swaptotal)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_swap_total = historical_data_read
        .iter()
        .map(|meminfo| meminfo.swaptotal)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.swaptotal / 1024_f64)),
            0.0,
            GREEN,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "swap total",
            min_swap_total / 1024_f64,
            max_swap_total / 1024_f64,
            latest.swaptotal / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // swap used
    let min_swap_used = historical_data_read
        .iter()
        .map(|meminfo| meminfo.swaptotal - meminfo.swapfree)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_swap_used = historical_data_read
        .iter()
        .map(|meminfo| meminfo.swaptotal - meminfo.swapfree)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.swaptotal - meminfo.swapfree) / 1024_f64,
                )
            }),
            0.0,
            RED,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "swap used",
            min_swap_used / 1024_f64,
            max_swap_used / 1024_f64,
            (latest.swaptotal - latest.swapfree) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
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

fn active_inactive_mem_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
) {
    let historical_data_read = HISTORY.memory.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|meminfo| meminfo.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|meminfo| meminfo.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.memtotal * 1.1_f64) / 1024_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let latest = historical_data_read.back().unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            "active/inactive memory",
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_label_formatter(&|size| {
            if size < &1024_f64 {
                format!("{:5.0} MB", size)
            } else if size < &10240_f64 {
                format!("{:5.1} GB", size / 1024_f64)
            } else {
                format!("{:5.0} GB", size / 1024_f64)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|meminfo| (meminfo.timestamp, meminfo.swaptotal)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:>10} {:>10} {:>10}",
            "", "min MB", "max MB", "last MB"
        ));
    // swap total; this is the total, so it doesn't need to be stacked.
    let min_mem_total = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memtotal)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_mem_total = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memtotal)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.memtotal / 1024_f64)),
            0.0,
            Palette99::pick(1),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "memory total",
            min_mem_total / 1024_f64,
            max_mem_total / 1024_f64,
            latest.memtotal / 1024_f64
        ))
        .legend(move |(x, y)| {
            Rectangle::new(
                [(x - 3, y - 3), (x + 3, y + 3)],
                Palette99::pick(1).filled(),
            )
        });
    // memory areas:
    // Hugepages total memory: hugepages_size * hugepages_total
    //
    // memfree
    //
    // SUnreclaim: slab unreclaimable   -- SLAB
    // SReclaimable: slab reclaimable
    // Active(anon): active anonymous   -- anonymous
    // Inactive(anon): inactive anonymous
    // Active(file): active file        -- file
    // Inactive(file): inactive file
    //
    // hugepages
    let min_hugepages = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.hugepagesize * meminfo.hugepages_total))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_hugepages = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.hugepagesize * meminfo.hugepages_total))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    ((meminfo.hugepagesize * meminfo.hugepages_total)
                        + meminfo.memfree
                        + meminfo.sreclaimable
                        + meminfo.inactive_anon
                        + meminfo.inactive_file
                        + meminfo.sunreclaim
                        + meminfo.active_anon
                        + meminfo.active_file)
                        / 1024_f64,
                )
            }),
            0.0,
            GREY.filled(),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "hugepages",
            min_hugepages / 1024_f64,
            max_hugepages / 1024_f64,
            (latest.hugepagesize * latest.hugepages_total) / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREY));
    // active_anon
    let min_active_anon = historical_data_read
        .iter()
        .map(|meminfo| meminfo.active_anon)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_active_anon = historical_data_read
        .iter()
        .map(|meminfo| meminfo.active_anon)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.inactive_file
                        + meminfo.active_file
                        + meminfo.sreclaimable
                        + meminfo.sunreclaim
                        + meminfo.inactive_anon
                        + meminfo.active_anon)
                        / 1024_f64,
                )
            }),
            0.0,
            BROWN_500.filled(),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "active anonymous",
            min_active_anon / 1024_f64,
            max_active_anon / 1024_f64,
            latest.active_anon / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BROWN_500.filled()));
    // inactive_anon
    let min_inactive_anon = historical_data_read
        .iter()
        .map(|meminfo| meminfo.inactive_anon)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_inactive_anon = historical_data_read
        .iter()
        .map(|meminfo| meminfo.inactive_anon)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.inactive_file
                        + meminfo.active_file
                        + meminfo.sreclaimable
                        + meminfo.sunreclaim
                        + meminfo.inactive_anon)
                        / 1024_f64,
                )
            }),
            0.0,
            BROWN_400.filled(),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "inactive anonymous",
            min_inactive_anon / 1024_f64,
            max_inactive_anon / 1024_f64,
            latest.inactive_anon / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BROWN_400.filled()));
    // sunreclaim
    let min_sunreclaim = historical_data_read
        .iter()
        .map(|meminfo| meminfo.sunreclaim)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_sunreclaim = historical_data_read
        .iter()
        .map(|meminfo| meminfo.sunreclaim)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.inactive_file
                        + meminfo.active_file
                        + meminfo.sreclaimable
                        + meminfo.sunreclaim)
                        / 1024_f64,
                )
            }),
            0.0,
            PURPLE_800.filled(),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "slab unreclaimable",
            min_sunreclaim / 1024_f64,
            max_sunreclaim / 1024_f64,
            latest.sunreclaim / 1024_f64
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE_800.filled())
        });
    // sreclaimable
    let min_sreclaim = historical_data_read
        .iter()
        .map(|meminfo| meminfo.sreclaimable)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_sreclaim = historical_data_read
        .iter()
        .map(|meminfo| meminfo.sreclaimable)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree
                        + meminfo.inactive_file
                        + meminfo.active_file
                        + meminfo.sreclaimable)
                        / 1024_f64,
                )
            }),
            0.0,
            PURPLE_100.filled(),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "slab reclaimable",
            min_sreclaim / 1024_f64,
            max_sreclaim / 1024_f64,
            latest.sreclaimable / 1024_f64
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE_100.filled())
        });
    // active_file
    let min_active_file = historical_data_read
        .iter()
        .map(|meminfo| meminfo.active_file)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_active_file = historical_data_read
        .iter()
        .map(|meminfo| meminfo.active_file)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree + meminfo.inactive_file + meminfo.active_file) / 1024_f64,
                )
            }),
            0.0,
            AMBER_700.filled(),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "active file",
            min_active_file / 1024_f64,
            max_active_file / 1024_f64,
            latest.active_file / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], AMBER_700.filled()));
    // inactive_file
    let min_inactive_file = historical_data_read
        .iter()
        .map(|meminfo| meminfo.inactive_file)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_inactive_file = historical_data_read
        .iter()
        .map(|meminfo| meminfo.inactive_file)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read.iter().map(|meminfo| {
                (
                    meminfo.timestamp,
                    (meminfo.memfree + meminfo.inactive_file) / 1024_f64,
                )
            }),
            0.0,
            AMBER_100.filled(),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "inactive file",
            min_inactive_file / 1024_f64,
            max_inactive_file / 1024_f64,
            latest.inactive_file / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], AMBER_100.filled()));
    // memfree
    let min_free = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memfree)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_free = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memfree)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.memfree / 1024_f64)),
            0.0,
            LIGHTGREEN.filled(),
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "memfree",
            min_free / 1024_f64,
            max_free / 1024_f64,
            latest.memfree / 1024_f64
        ))
        .legend(move |(x, y)| {
            Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN.filled())
        });
    // memavailable
    let min_memavailable = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memavailable)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_memavailable = historical_data_read
        .iter()
        .map(|meminfo| meminfo.memavailable)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.memavailable / 1024_f64)),
            ShapeStyle {
                color: RED.into(),
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "memavailable",
            min_memavailable / 1024_f64,
            max_memavailable / 1024_f64,
            latest.memavailable / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
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

fn committed_mem_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
) {
    let historical_data_read = HISTORY.memory.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|meminfo| meminfo.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|meminfo| meminfo.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value_committed_as = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.committed_as * 1.1_f64) / 1024_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_commitlimit = historical_data_read
        .iter()
        .map(|meminfo| (meminfo.commitlimit * 1.1_f64) / 1024_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value = high_value_committed_as.max(high_value_commitlimit);
    let latest = historical_data_read.back().unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            "Committed memory overview",
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_label_formatter(&|size| {
            if size < &1024_f64 {
                format!("{:5.0} MB", size)
            } else if size < &10240_f64 {
                format!("{:5.1} GB", size / 1024_f64)
            } else {
                format!("{:5.0} GB", size / 1024_f64)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|meminfo| (meminfo.timestamp, meminfo.swaptotal)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:>10} {:>10} {:>10}",
            "", "min MB", "max MB", "last MB"
        ));
    // committed_as
    let min_committed_as = historical_data_read
        .iter()
        .map(|meminfo| meminfo.committed_as)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_committed_as = historical_data_read
        .iter()
        .map(|meminfo| meminfo.committed_as)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.committed_as / 1024_f64)),
            0.0,
            BLUE,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "committed_AS",
            min_committed_as / 1024_f64,
            max_committed_as / 1024_f64,
            latest.committed_as / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE.filled()));
    // commitlimit used
    let min_commitlimit = historical_data_read
        .iter()
        .map(|meminfo| meminfo.commitlimit)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_commitlimit = historical_data_read
        .iter()
        .map(|meminfo| meminfo.commitlimit)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .map(|meminfo| (meminfo.timestamp, meminfo.commitlimit / 1024_f64)),
            ShapeStyle {
                color: BLACK.into(),
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "commitlimit",
            min_commitlimit / 1024_f64,
            max_commitlimit / 1024_f64,
            latest.commitlimit / 1024_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
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
