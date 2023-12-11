use proc_sys_parser::{schedstat, stat, meminfo, diskstats, net_dev};

#[derive(Debug)]
pub struct ProcData
{
    pub stat: stat::ProcStat,
    pub schedstat: schedstat::ProcSchedStat,
    pub meminfo: meminfo::ProcMemInfo,
    pub diskstats: diskstats::ProcDiskStats,
    pub net_dev: net_dev::ProcNetDev,
}

pub async fn read_proc_data() -> ProcData
{
    let proc_stat = stat::read();
    let proc_schedstat = schedstat::read();
    let proc_meminfo = meminfo::read();
    let proc_diskstats = diskstats::read();
    let proc_netdev = net_dev::read();
    ProcData {
        stat: proc_stat,
        schedstat: proc_schedstat,
        meminfo: proc_meminfo,
        diskstats: proc_diskstats,
        net_dev: proc_netdev,
    }
}