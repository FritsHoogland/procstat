use chrono::{DateTime, Local};
use std::collections::HashMap;
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::prelude::{AreaSeries, BLACK, LineSeries, Palette99, RED, ShapeStyle, TRANSPARENT, WHITE};
use plotters::prelude::full_palette::LIGHTGREEN;
use crate::common::{ProcData, single_statistic_u64, Statistic};
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};
use crate::{GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH};
use crate::add_list_of_u64_data_to_statistics;
use sysctl::{Ctl, Sysctl};
use crate::pressure::pressure_memory_plot;

#[derive(Debug)]
pub struct MemInfo {
    pub timestamp: DateTime<Local>,
    pub memfree: f64,
    pub memavailable: f64,
    pub memtotal: f64,
    pub buffers: f64,
    pub cached: f64,
    pub swapcached: f64,
    pub kernelstack: f64,
    pub hardwarecorrupted: f64,
    pub slab: f64,
    pub pagetables: f64,
    pub dirty: f64,
    pub shmem: f64,
    pub mapped: f64,
    pub anonpages: f64,
    pub hugepages_total: f64,
    pub hugepages_free: f64,
    pub hugepagesize: f64,
}

pub async fn process_meminfo_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>) {
    add_list_of_u64_data_to_statistics!(meminfo, "", proc_data.timestamp, proc_data, meminfo, statistics, memtotal, memfree, memavailable, buffers, cached, swapcached, active, inactive, active_anon, inactive_anon, active_file, inactive_file, unevictable, mlocked, swaptotal, swapfree, zswap, zswapped, dirty, writeback, anonpages, mapped, shmem, kreclaimable, slab, sreclaimable, sunreclaim, kernelstack, shadowcallstack, pagetables, secpagetables, nfs_unstable, bounce, writebacktmp, commitlimit, committed_as, vmalloctotal, vmallocused, vmallocchunk, percpu, hardwarecorrupted, anonhugepages, shmemhugepages, shmempmdmapped, filehugepages, filepmdmapped, cmatotal, cmafree, hugepages_total, hugepages_free, hugepages_rsvd, hugepages_surp, hugepagesize, hugetlb);
}

pub async fn print_meminfo(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
    print_header: bool
)
{
    if print_header {
        match output {
            "sar-r" => {
                println!("{:10}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                         "Timestamp",
                         "mbmemfree",
                         "mbavail",
                         "mbmemused",
                         "%memused",
                         "mbbuffers",
                         "mbcached",
                         "mbcommit",
                         "%commit",
                         "mbactive",
                         "mbinact",
                         "mbdirty",
                );
            },
            "sar-r-ALL" => {
                println!("{:10}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                         "Timestamp",
                         "mbmemfree",
                         "mbavail",
                         "mbmemused",
                         "%memused",
                         "mbbuffers",
                         "mbcached",
                         "mbcommit",
                         "%commit",
                         "mbactive",
                         "mbinact",
                         "mbdirty",
                         "mbanonpg",
                         "mbslab",
                         "mbstack",
                         "mbpgtbl",
                         "mbvmused",
                );
            },
            "sar-H" => {
                println!("{:10}    {:>10} {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp",
                    "mbhugfree",
                    "mbhugused",
                    "%hugused",
                    "mbhugrsvd",
                    "mbhugsurp",
                );
            },
            "sar-S" => {
                println!("{:10}    {:>10} {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp",
                    "mbswpfree",
                    "mbswpused",
                    "%swpused",
                    "mbswpcad",
                    "%swpcad",
                );
            },
            &_ => todo!(),
        }
    }

    let timestamp = statistics.get(&("meminfo".to_string(), "".to_string(), "memfree".to_string())).unwrap().last_timestamp;
    let memfree = statistics.get(&("meminfo".to_string(), "".to_string(), "memfree".to_string())).unwrap().last_value;
    let memavailable = statistics.get(&("meminfo".to_string(), "".to_string(), "memavailable".to_string())).unwrap().last_value;
    let memtotal = statistics.get(&("meminfo".to_string(), "".to_string(), "memtotal".to_string())).unwrap().last_value;
    let buffers = statistics.get(&("meminfo".to_string(), "".to_string(), "buffers".to_string())).unwrap().last_value;
    let cached = statistics.get(&("meminfo".to_string(), "".to_string(), "cached".to_string())).unwrap().last_value;
    let committed_as = statistics.get(&("meminfo".to_string(), "".to_string(), "committed_as".to_string())).unwrap().last_value;
    let swaptotal = statistics.get(&("meminfo".to_string(), "".to_string(), "swaptotal".to_string())).unwrap().last_value;
    let active = statistics.get(&("meminfo".to_string(), "".to_string(), "active".to_string())).unwrap().last_value;
    let inactive = statistics.get(&("meminfo".to_string(), "".to_string(), "inactive".to_string())).unwrap().last_value;
    let dirty = statistics.get(&("meminfo".to_string(), "".to_string(), "dirty".to_string())).unwrap().last_value;
    let anonpages = statistics.get(&("meminfo".to_string(), "".to_string(), "anonpages".to_string())).unwrap().last_value;
    let slab = statistics.get(&("meminfo".to_string(), "".to_string(), "slab".to_string())).unwrap().last_value;
    let kernelstack = statistics.get(&("meminfo".to_string(), "".to_string(), "kernelstack".to_string())).unwrap().last_value;
    let pagetables = statistics.get(&("meminfo".to_string(), "".to_string(), "pagetables".to_string())).unwrap().last_value;
    let vmallocused = statistics.get(&("meminfo".to_string(), "".to_string(), "vmallocused".to_string())).unwrap().last_value;
    let hugepages_total = statistics.get(&("meminfo".to_string(), "".to_string(), "hugepages_total".to_string())).unwrap().last_value;
    let hugepages_free = statistics.get(&("meminfo".to_string(), "".to_string(), "hugepages_free".to_string())).unwrap().last_value;
    let hugepagesize = statistics.get(&("meminfo".to_string(), "".to_string(), "hugepagesize".to_string())).unwrap().last_value;
    let hugepages_reserved = statistics.get(&("meminfo".to_string(), "".to_string(), "hugepages_rsvd".to_string())).unwrap().last_value;
    let hugepages_surplus = statistics.get(&("meminfo".to_string(), "".to_string(), "hugepages_surp".to_string())).unwrap().last_value;
    let swap_free = statistics.get(&("meminfo".to_string(), "".to_string(), "swapfree".to_string())).unwrap().last_value;
    let swap_total = statistics.get(&("meminfo".to_string(), "".to_string(), "swaptotal".to_string())).unwrap().last_value;
    let swap_cached = statistics.get(&("meminfo".to_string(), "".to_string(), "swapcached".to_string())).unwrap().last_value;
    // this is what sar defines as non-used memory; see: https://github.com/sysstat/sysstat/blob/499f5b153e9707892bb8841d37e6ed3a0aa617e2/pr_stats.c#L809
    let mut non_used_memory = memfree + buffers + cached + slab;
    if non_used_memory > memtotal { non_used_memory = memtotal };

    match output {
        // https://github.com/sysstat/sysstat/blob/499f5b153e9707892bb8841d37e6ed3a0aa617e2/pr_stats.c#L789
        "sar-r" => {
            println!("{:10}    {:10.0} {:10.0} {:10.0} {:10.2} {:10.0} {:10.0} {:10.0} {:10.2} {:10.0} {:10.0} {:10.0}",
                timestamp.format("%H:%M:%S"),
                memfree / 1024_f64,
                memavailable / 1024_f64,
                (memtotal - non_used_memory) / 1024_f64,
                (memtotal - non_used_memory) / memtotal * 100_f64,
                buffers / 1024_f64,
                cached / 1024_f64,
                committed_as / 1024_f64,
                committed_as / (memtotal + swaptotal) * 100_f64,
                active / 1024_f64,
                inactive / 1024_f64,
                dirty / 1024_f64,
            );
        },
        "sar-r-ALL" => {
            println!("{:10}    {:10.0} {:10.0} {:10.0} {:10.2} {:10.0} {:10.0} {:10.0} {:10.2} {:10.0} {:10.0} {:10.0} {:10.0} {:10.0} {:10.0} {:10.0} {:10.0}",
                timestamp.format("%H:%M:%S"),
                memfree / 1024_f64,
                memavailable / 1024_f64,
                (memtotal - non_used_memory) / 1024_f64,
                (memtotal - non_used_memory) / memtotal * 100_f64,
                buffers / 1024_f64,
                cached / 1024_f64,
                committed_as / 1024_f64,
                committed_as / (memtotal + swaptotal) * 100_f64,
                active / 1024_f64,
                inactive / 1024_f64,
                dirty / 1024_f64,
                anonpages / 1024_f64,
                slab / 1024_f64,
                kernelstack / 1024_f64,
                pagetables / 1024_f64,
                vmallocused / 1024_f64,
            );
        },
        "sar-H" => {
            println!("{:10}    {:10.0} {:10.0} {:10.2} {:10.0} {:10.0}",
                timestamp.format("%H:%M:%S"),
                (hugepages_free * hugepagesize ) / (1024_f64 * 1024_f64),
                ((hugepages_total - hugepages_free) * hugepagesize ) / (1024_f64 * 1024_f64),
                if hugepages_total == 0_f64 { 0_f64 } else { (hugepages_total - hugepages_free) / hugepages_total * 100_f64 },
                (hugepages_reserved * hugepagesize ) / (1024_f64 * 1024_f64),
                (hugepages_surplus * hugepagesize ) / (1024_f64 * 1024_f64),
            );
        },
        "sar-S" => {
            println!("{:10}    {:10.0} {:10.0} {:10.2} {:10.0} {:10.0}",
                timestamp.format("%H:%M:%S"),
                swap_free / 1024_f64,
                (swap_total - swap_free) / 1024_f64,
                if swap_total == 0_f64 { 0_f64 } else { (swap_total - swap_free) / swap_total * 100_f64 },
                swap_cached / 1024_f64,
                if swap_total - swap_free == 0_f64 { 0_f64} else { swap_cached / (swap_total - swap_free) * 100_f64 },
            );
        },
        &_ => todo!(),
    }
}

pub async fn add_memory_to_history(statistics: &HashMap<(String, String, String), Statistic>)
{
    if !statistics.get(&("meminfo".to_string(), "".to_string(), "memtotal".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("meminfo".to_string(), "".to_string(), "memtotal".to_string())).unwrap().last_timestamp;
    let memfree = statistics.get(&("meminfo".to_string(), "".to_string(), "memfree".to_string())).unwrap().last_value;
    let memavailable = statistics.get(&("meminfo".to_string(), "".to_string(), "memavailable".to_string())).unwrap().last_value;
    let memtotal = statistics.get(&("meminfo".to_string(), "".to_string(), "memtotal".to_string())).unwrap().last_value;
    let buffers = statistics.get(&("meminfo".to_string(), "".to_string(), "buffers".to_string())).unwrap().last_value;
    let cached = statistics.get(&("meminfo".to_string(), "".to_string(), "cached".to_string())).unwrap().last_value;
    let swapcached = statistics.get(&("meminfo".to_string(), "".to_string(), "swapcached".to_string())).unwrap().last_value;
    let kernelstack = statistics.get(&("meminfo".to_string(), "".to_string(), "kernelstack".to_string())).unwrap().last_value;
    let hardwarecorrupted= statistics.get(&("meminfo".to_string(), "".to_string(), "hardwarecorrupted".to_string())).unwrap().last_value;
    let slab = statistics.get(&("meminfo".to_string(), "".to_string(), "slab".to_string())).unwrap().last_value;
    let pagetables = statistics.get(&("meminfo".to_string(), "".to_string(), "pagetables".to_string())).unwrap().last_value;
    let dirty = statistics.get(&("meminfo".to_string(), "".to_string(), "dirty".to_string())).unwrap().last_value;
    let shmem = statistics.get(&("meminfo".to_string(), "".to_string(), "shmem".to_string())).unwrap().last_value;
    let mapped = statistics.get(&("meminfo".to_string(), "".to_string(), "mapped".to_string())).unwrap().last_value;
    let anonpages= statistics.get(&("meminfo".to_string(), "".to_string(), "anonpages".to_string())).unwrap().last_value;
    let hugepages_total = statistics.get(&("meminfo".to_string(), "".to_string(), "hugepages_total".to_string())).unwrap().last_value;
    let hugepages_free = statistics.get(&("meminfo".to_string(), "".to_string(), "hugepages_free".to_string())).unwrap().last_value;
    let hugepagesize = statistics.get(&("meminfo".to_string(), "".to_string(), "hugepagesize".to_string())).unwrap().last_value;
    HISTORY.memory.write().unwrap().push_back( MemInfo {
        timestamp,
        memfree,
        memavailable,
        memtotal,
        buffers,
        cached,
        swapcached,
        kernelstack,
        hardwarecorrupted,
        slab,
        pagetables,
        dirty,
        shmem,
        mapped,
        anonpages,
        hugepages_total,
        hugepages_free,
        hugepagesize,
    });
}


pub fn create_memory_plot(
    buffer: &mut [u8]
)
{
    let backend = BitMapBackend::with_buffer(buffer, (GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((1, 1));
    memory_plot(&mut multi_backend, 0);
}

pub fn create_memory_psi_plot(
    buffer: &mut [u8]
)
{
    let backend = BitMapBackend::with_buffer(buffer, (GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    pressure_memory_plot(&mut multi_backend, 1);
}

fn memory_plot(
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
        .map(|meminfo| (meminfo.memtotal * 1.1_f64) / 1024_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let latest = historical_data_read
        .back()
        .unwrap();
    let min_free_kbytes: f64 = Ctl::new("vm.min_free_kbytes").unwrap().description().unwrap_or_default().parse::<f64>().unwrap_or_default();

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
        .y_desc("Memory MB")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // colour picker
    let mut palette99_pick = 1_usize;
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|meminfo| (meminfo.timestamp, meminfo.memtotal)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // memory total; this is the total limit, so it doesn't need to be stacked.
    let min_memory_total = historical_data_read.iter().map(|meminfo| meminfo.memtotal).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_memory_total = historical_data_read.iter().map(|meminfo| meminfo.memtotal).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, meminfo.memtotal/1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "memory total", min_memory_total/1024_f64, max_memory_total/1024_f64, latest.memtotal/1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // The next memory areas should be stacked, so the 'top' memory allocation should be shown first, then one allocation removed, etc.
    // This is a manually constructed stacked area graph.
    // First hugepages used.
    // hugepages_used, hugepages_free, buffers, swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, cached, mapped, anonymous, memfree
    let min_hugepages_used = historical_data_read.iter().map(|meminfo| (meminfo.hugepages_total - meminfo.hugepages_free) * meminfo.hugepagesize).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_hugepages_used = historical_data_read.iter().map(|meminfo| (meminfo.hugepages_total - meminfo.hugepages_free) * meminfo.hugepagesize).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (((meminfo.hugepages_total - meminfo.hugepages_free) * meminfo.hugepagesize) + meminfo.buffers + meminfo.swapcached + meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "hugepages used", min_hugepages_used/1024_f64, max_hugepages_used/1024_f64, ((latest.hugepages_total - latest.hugepages_free)*latest.hugepagesize)/1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // hugepages_free, buffers, swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_hugepages_free = historical_data_read.iter().map(|meminfo| meminfo.hugepages_free * meminfo.hugepagesize).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_hugepages_free = historical_data_read.iter().map(|meminfo| meminfo.hugepages_free * meminfo.hugepagesize).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.hugepages_free * meminfo.hugepagesize) + meminfo.buffers + meminfo.swapcached + meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "hugepages free", min_hugepages_free/1024_f64, max_hugepages_free/1024_f64, (latest.hugepages_free * latest.hugepagesize)/1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // buffers, swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_buffers = historical_data_read.iter().map(|meminfo| meminfo.buffers).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_buffers = historical_data_read.iter().map(|meminfo| meminfo.buffers).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.buffers + meminfo.swapcached + meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "buffers", min_buffers/1024_f64, max_buffers/1024_f64, latest.buffers / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // swapcached, kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_swapcached = historical_data_read.iter().map(|meminfo| meminfo.swapcached).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_swapcached = historical_data_read.iter().map(|meminfo| meminfo.swapcached).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.swapcached + meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "swapcached", min_swapcached/1024_f64, max_swapcached/1024_f64, latest.swapcached / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // kernelstack, hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_kernelstack = historical_data_read.iter().map(|meminfo| meminfo.kernelstack).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_kernelstack = historical_data_read.iter().map(|meminfo| meminfo.kernelstack).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.kernelstack + meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "kernelstack", min_kernelstack/1024_f64, max_kernelstack/1024_f64, latest.kernelstack / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // hardwarecorrupted, slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_hardwarecorrupted = historical_data_read.iter().map(|meminfo| meminfo.hardwarecorrupted).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_hardwarecorrupted = historical_data_read.iter().map(|meminfo| meminfo.hardwarecorrupted).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.hardwarecorrupted + meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "hardwarecorrupted", min_hardwarecorrupted/1024_f64, max_hardwarecorrupted/1024_f64, latest.hardwarecorrupted / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // slab, pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_slab = historical_data_read.iter().map(|meminfo| meminfo.slab).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_slab = historical_data_read.iter().map(|meminfo| meminfo.slab).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.slab + meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "slab", min_slab/1024_f64, max_slab/1024_f64, latest.slab / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // pagetables, dirty, shmem, mapped, cached, anonymous, memfree
    let min_pagetables = historical_data_read.iter().map(|meminfo| meminfo.pagetables).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_pagetables = historical_data_read.iter().map(|meminfo| meminfo.pagetables).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.pagetables + meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pagetables", min_pagetables/1024_f64, max_pagetables/1024_f64, latest.pagetables / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // dirty sits inside 'cached'
    // dirty, shmem, mapped, cached, anonymous, memfree
    let min_dirty = historical_data_read.iter().map(|meminfo| meminfo.dirty).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_dirty = historical_data_read.iter().map(|meminfo| meminfo.dirty).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.cached + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "dirty", min_dirty/1024_f64, max_dirty/1024_f64, latest.dirty / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // shmem, mapped, cached, anonymous, memfree
    let min_shmem = historical_data_read.iter().map(|meminfo| meminfo.shmem).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_shmem = historical_data_read.iter().map(|meminfo| meminfo.shmem).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.cached - meminfo.dirty) + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "shmem (smallpages)", min_shmem/1024_f64, max_shmem/1024_f64, latest.shmem / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // mapped, cached, anonymous, memfree
    let min_mapped = historical_data_read.iter().map(|meminfo| meminfo.mapped).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_mapped = historical_data_read.iter().map(|meminfo| meminfo.mapped).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.cached - meminfo.dirty - meminfo.shmem) + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "mapped", min_mapped/1024_f64, max_mapped/1024_f64, latest.mapped / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // cached, anonymous, memfree
    let min_cached = historical_data_read.iter().map(|meminfo| meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_cached = historical_data_read.iter().map(|meminfo| meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, ((meminfo.cached - meminfo.mapped.max(meminfo.shmem) - meminfo.dirty) + meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "cached", min_cached/1024_f64, max_cached/1024_f64, (latest.cached - latest.mapped.max(latest.shmem) - latest.dirty) / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // anonymous, memfree
    let min_anonpages = historical_data_read.iter().map(|meminfo| meminfo.anonpages).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_anonpages = historical_data_read.iter().map(|meminfo| meminfo.anonpages).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (meminfo.anonpages + meminfo.memfree) / 1024_f64)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "anonpages", min_anonpages/1024_f64, max_anonpages/1024_f64, latest.anonpages / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    //palette99_pick += 1;
    // memfree
    let min_memfree = historical_data_read.iter().map(|meminfo| meminfo.memfree).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_memfree = historical_data_read.iter().map(|meminfo| meminfo.memfree).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, meminfo.memfree / 1024_f64)), 0.0, LIGHTGREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "memfree", min_memfree/1024_f64, max_memfree/1024_f64, latest.memfree / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN.filled()));
    //palette99_pick += 1;
    // special drawing
    // memavailable
    let min_memavailable = historical_data_read.iter().map(|meminfo| meminfo.memavailable).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_memavailable = historical_data_read.iter().map(|meminfo| meminfo.memavailable).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, meminfo.memavailable / 1024_f64)),  ShapeStyle { color: RED.into(), filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "memavailable", min_memavailable/1024_f64, max_memavailable/1024_f64, latest.memavailable / 1024_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    //
    // min_free_kbytes / pages_min
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, min_free_kbytes / 1024_f64)),  ShapeStyle { color: BLACK.into(), filled: false, stroke_width: 1} )).unwrap();
    // pages_low
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (min_free_kbytes+(min_free_kbytes/4_f64)) / 1024_f64)),  ShapeStyle { color: BLACK.into(), filled: false, stroke_width: 1} )).unwrap();
    // pages_high
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|meminfo| (meminfo.timestamp, (min_free_kbytes+(min_free_kbytes/2_f64)) / 1024_f64)),  ShapeStyle { color: BLACK.into(), filled: false, stroke_width: 1} )).unwrap();
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
