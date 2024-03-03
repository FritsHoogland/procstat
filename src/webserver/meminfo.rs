use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::prelude::{AreaSeries, BLACK, LineSeries, Palette99, RED, ShapeStyle, TRANSPARENT, WHITE};
use plotters::prelude::full_palette::LIGHTGREEN;

use crate::webserver::vmstat::swap_inout_plot;
use crate::webserver::pressure::pressure_memory_plot;
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};
use sysctl::{Ctl, Sysctl};
use crate::ARGS;

pub fn create_memory_plot(
    buffer: &mut [u8]
)
{
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((1, 1));
    memory_plot(&mut multi_backend, 0);
}

pub fn create_memory_psi_plot(
    buffer: &mut [u8]
)
{
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    pressure_memory_plot(&mut multi_backend, 1);
}

pub fn create_memory_swap_plot( buffer: &mut [u8]) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    swap_space_plot(&mut multi_backend, 1);
}

pub fn create_memory_swap_inout_plot(
    buffer: &mut [u8]
)
{
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    memory_plot(&mut multi_backend, 0);
    swap_inout_plot(&mut multi_backend, 1);
    swap_space_plot(&mut multi_backend, 2);
}
pub fn memory_plot(
    multi_backend: &mut  [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
)
{
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
    let latest = historical_data_read
        .back();
    let min_free_kbytes: f64 = match Ctl::new("vm.min_free_kbytes") {
        Ok(value) => value.description().unwrap_or_default().parse::<f64>().unwrap_or_default(),
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
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_label_formatter(&|size| {
                 if size < &1024_f64  { format!("{:5.0} MB", size) }
            else if size < &10240_f64 { format!("{:5.1} GB", size / 1024_f64) }
            else                      { format!("{:5.0} GB", size / 1024_f64) }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // colour picker
    let mut palette99_pick = 1_usize;
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|meminfo| (meminfo.timestamp, meminfo.memtotal)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min MB", "max MB", "last MB"));
    //
    // memory total; this is the total limit, so it doesn't need to be stacked.
    let min_memory_total = historical_data_read.iter().map(|meminfo| meminfo.memtotal).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_memory_total = historical_data_read.iter().map(|meminfo| meminfo.memtotal).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, meminfo.memtotal/1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "memory total", min_memory_total/1024_f64, max_memory_total/1024_f64, latest.map_or(0_f64, |latest| latest.memtotal/1024_f64)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // The next memory areas should be stacked, so the 'top' memory allocation should be shown first, then one allocation removed, etc.
    // This is a manually constructed stacked area graph.
    // First hugepages used.
    // hugepages_used, hugepages_free (=hugepages_total), buffers, swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, cached, mapped, anonymous, memfree
    let min_hugepages_used = historical_data_read.iter().map(|meminfo| (meminfo.hugepages_total - meminfo.hugepages_free) * meminfo.hugepagesize).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_hugepages_used = historical_data_read.iter().map(|meminfo| (meminfo.hugepages_total - meminfo.hugepages_free) * meminfo.hugepagesize).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    // issue: https://github.com/FritsHoogland/procstat/issues/1: hugepages_used should be
    // hugepages_total, so that hugepages_free can overwrite all that is free.
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.hugepages_total * meminfo.hugepagesize) + meminfo.buffers + meminfo.swapcached + meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "hugepages used", min_hugepages_used/1024_f64, max_hugepages_used/1024_f64, ((latest.map_or(0_f64, |latest| latest.hugepages_total) - latest.map_or(0_f64, |latest| latest.hugepages_free)) * latest.map_or(0_f64, |latest| latest.hugepagesize))/1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // hugepages_free, buffers, swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_hugepages_free = historical_data_read.iter().map(|meminfo| meminfo.hugepages_free * meminfo.hugepagesize).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_hugepages_free = historical_data_read.iter().map(|meminfo| meminfo.hugepages_free * meminfo.hugepagesize).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.hugepages_free * meminfo.hugepagesize) + meminfo.buffers + meminfo.swapcached + meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "hugepages free", min_hugepages_free/1024_f64, max_hugepages_free/1024_f64, (latest.map_or(0_f64, |latest| latest.hugepages_free) * latest.map_or(0_f64, |latest| latest.hugepagesize))/1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // buffers, swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_buffers = historical_data_read.iter().map(|meminfo| meminfo.buffers).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_buffers = historical_data_read.iter().map(|meminfo| meminfo.buffers).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.buffers + meminfo.swapcached + meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "buffers", min_buffers/1024_f64, max_buffers/1024_f64, latest.map_or(0_f64, |latest| latest.buffers) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_swapcached = historical_data_read.iter().map(|meminfo| meminfo.swapcached).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_swapcached = historical_data_read.iter().map(|meminfo| meminfo.swapcached).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.swapcached + meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "swapcached", min_swapcached/1024_f64, max_swapcached/1024_f64, latest.map_or(0_f64, |latest| latest.swapcached) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_kernelstack = historical_data_read.iter().map(|meminfo| meminfo.kernelstack).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_kernelstack = historical_data_read.iter().map(|meminfo| meminfo.kernelstack).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "kernelstack", min_kernelstack/1024_f64, max_kernelstack/1024_f64, latest.map_or(0_f64, |latest| latest.kernelstack) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_hardwarecorrupted = historical_data_read.iter().map(|meminfo| meminfo.hardwarecorrupted).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_hardwarecorrupted = historical_data_read.iter().map(|meminfo| meminfo.hardwarecorrupted).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "hardwarecorrupted", min_hardwarecorrupted/1024_f64, max_hardwarecorrupted/1024_f64, latest.map_or(0_f64, |latest| latest.hardwarecorrupted) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_slab = historical_data_read.iter().map(|meminfo| meminfo.slab).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_slab = historical_data_read.iter().map(|meminfo| meminfo.slab).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "slab", min_slab/1024_f64, max_slab/1024_f64, latest.map_or(0_f64, |latest| latest.slab) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_pagetables = historical_data_read.iter().map(|meminfo| meminfo.pagetables).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_pagetables = historical_data_read.iter().map(|meminfo| meminfo.pagetables).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pagetables", min_pagetables/1024_f64, max_pagetables/1024_f64, latest.map_or(0_f64, |latest| latest.pagetables) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // dirty sits inside 'cached'
    // dirty, shmem, mapped, cached, anonymous, memfree
    let min_dirty = historical_data_read.iter().map(|meminfo| meminfo.dirty).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_dirty = historical_data_read.iter().map(|meminfo| meminfo.dirty).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "dirty", min_dirty/1024_f64, max_dirty/1024_f64, latest.map_or(0_f64, |latest| latest.dirty) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // shmem, mapped, cached, anonymous, memfree
    let min_shmem = historical_data_read.iter().map(|meminfo| meminfo.shmem).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_shmem = historical_data_read.iter().map(|meminfo| meminfo.shmem).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.cached - meminfo.dirty) + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "shmem (smallpages)", min_shmem/1024_f64, max_shmem/1024_f64, latest.map_or(0_f64, |latest| latest.shmem) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // mapped, cached, anonymous, memfree
    let min_mapped = historical_data_read.iter().map(|meminfo| meminfo.mapped).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_mapped = historical_data_read.iter().map(|meminfo| meminfo.mapped).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.cached - meminfo.dirty - meminfo.shmem) + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "mapped", min_mapped/1024_f64, max_mapped/1024_f64, latest.map_or(0_f64, |latest| latest.mapped) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // cached, anonymous, memfree
    let min_cached = historical_data_read.iter().map(|meminfo| meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_cached = historical_data_read.iter().map(|meminfo| meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty) + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "cached", min_cached/1024_f64, max_cached/1024_f64, (latest.map_or(0_f64, |latest| latest.cached) - latest.map_or(0_f64, |latest| latest.mapped).max(latest.map_or(0_f64, |latest| latest.shmem)) - latest.map_or(0_f64, |latest| latest.dirty)) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // anonymous, memfree
    let min_anonpages = historical_data_read.iter().map(|meminfo| meminfo.anonpages).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_anonpages = historical_data_read.iter().map(|meminfo| meminfo.anonpages).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "anonpages", min_anonpages/1024_f64, max_anonpages/1024_f64, latest.map_or(0_f64, |latest| latest.anonpages) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    //palette99_pick += 1;
    // memfree
    let min_memfree = historical_data_read.iter().map(|meminfo| meminfo.memfree).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_memfree = historical_data_read.iter().map(|meminfo| meminfo.memfree).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(AreaSeries::new(historical_data_read
            .iter()
        .map(|meminfo| (meminfo.timestamp, meminfo.memfree / 1024_f64)), 0.0, LIGHTGREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "memfree", min_memfree/1024_f64, max_memfree/1024_f64, latest.map_or(0_f64, |latest| latest.memfree) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN.filled()));
    //palette99_pick += 1;
    // special drawing
    // memavailable
    let min_memavailable = historical_data_read.iter().map(|meminfo| meminfo.memavailable).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    let max_memavailable = historical_data_read.iter().map(|meminfo| meminfo.memavailable).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, meminfo.memavailable / 1024_f64)),  ShapeStyle { color: RED.into(), filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "memavailable", min_memavailable/1024_f64, max_memavailable/1024_f64, latest.map_or(0_f64, |latest| latest.memavailable) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    if min_free_kbytes != 0_f64 {
        // min_free_kbytes / pages_min
        contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, min_free_kbytes / 1024_f64)),  ShapeStyle { color: BLACK.into(), filled: false, stroke_width: 1} )).unwrap();
        // pages_low
        contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (min_free_kbytes+(min_free_kbytes/4_f64)) / 1024_f64)),  ShapeStyle { color: BLACK.into(), filled: false, stroke_width: 1} )).unwrap();
        // pages_high
        contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (min_free_kbytes+(min_free_kbytes/2_f64)) / 1024_f64)),  ShapeStyle { color: BLACK.into(), filled: false, stroke_width: 1} )).unwrap();
    }
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

fn swap_space_plot(
    multi_backend: &mut  [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
)
{
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
    let latest = historical_data_read
        .back()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Swap usage", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_label_formatter(&|size| {
                 if size < &1024_f64  { format!("{:5.0} MB", size) }
            else if size < &10240_f64 { format!("{:5.1} GB", size / 1024_f64) }
            else                      { format!("{:5.0} GB", size / 1024_f64) }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|meminfo| (meminfo.timestamp, meminfo.swaptotal)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min MB", "max MB", "last MB"));
    // swap total; this is the total, so it doesn't need to be stacked.
    let min_swap_total = historical_data_read.iter().map(|meminfo| meminfo.swaptotal).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_swap_total = historical_data_read.iter().map(|meminfo| meminfo.swaptotal).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, meminfo.swaptotal/1024_f64)), 0.0, GREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "swap total", min_swap_total/1024_f64, max_swap_total/1024_f64, latest.swaptotal/1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // swap used
    let min_swap_used = historical_data_read.iter().map(|meminfo| meminfo.swaptotal - meminfo.swapfree).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_swap_used = historical_data_read.iter().map(|meminfo| meminfo.swaptotal - meminfo.swapfree).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.swaptotal-meminfo.swapfree)/1024_f64)), 0.0, RED))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "swap used", min_swap_used/1024_f64, max_swap_used/1024_f64, (latest.swaptotal-latest.swapfree)/1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
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
