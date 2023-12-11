use proc_sys_parser::{schedstat, schedstat::ProcSchedStat, stat, stat::ProcStat};

#[derive(Debug)]
pub struct ProcData
{
    pub stat: ProcStat,
    pub schedstat: ProcSchedStat,
}

pub async fn read_proc_data() -> ProcData
{
    let proc_stat = stat::read();
    let proc_schedstat = schedstat::read();
    ProcData {
        stat: proc_stat,
        schedstat: proc_schedstat,
    }
}