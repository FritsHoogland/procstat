# procstat: show linux statistics from the proc filesystem

This is a utility that reads statistics from the `/proc` and `/sys` linux pseudo-filesystems and tries to show the information the most convenient and useful ways possible.

Because many people are very used to using tools like `sar` and `iostat`, this utility does provide output that is a copy of these tools. 
That means that you do not have to change your procedure for linux analysis, except for running another executable.
However, what `procstat` tries to do is provide more, better or different information using modern linux statistics.

By default, `procstat` shows CPU information in the same way that `sar` does:
```
$ procstat
Timestamp  cpu             %usr     %nice      %sys   %iowait    %steal     %idle
10:26:36   all             0.00      0.00      0.40      0.00      0.00     99.60
10:26:37   all             0.00      0.00      0.00      0.00      0.00    100.00
10:26:38   all             0.00      0.00      0.20      0.00      0.00     99.80
10:26:39   all             0.20      0.00      0.20      0.00      0.00     99.60
```
Nothing shocking here. 
However, I do like using absolute amount of CPU time better than percentages. `procstat` has the `cpu-all` option:
```
$ procstat -o cpu-all
Timestamp  cpu            usr_s    nice_s     sys_s  iowait_s   steal_s     irq_s    soft_s   guest_s   gnice_s    idle_s sched_r_s sched_w_s
10:40:21   all             0.00      0.00      0.00      0.00      0.00      0.00      0.00      0.00      0.00      4.99      0.01      0.00
10:40:22   all             0.01      0.00      0.01      0.00      0.00      0.00      0.00      0.00      0.00      4.98      0.02      0.00
10:40:23   all             0.00      0.00      0.03      0.00      0.00      0.00      0.00      0.00      0.00      4.97      0.03      0.00
10:40:24   all             0.00      0.00      0.00      0.00      0.00      0.00      0.00      0.00      0.00      4.99      0.01      0.00
10:40:25   all             0.01      0.00      0.01      0.00      0.00      0.00      0.01      0.00      0.00      4.98      0.02      0.00
10:40:26   all             0.01      0.00      0.01      0.00      0.00      0.00      0.00      0.00      0.00      4.99      0.02      0.00
```
(scroll to the right to see the idle time)
This is the reason why `procstat` is created.

Current output options:
- iostat
- iostat-x 
- sar-b
- sar-B
- sar-d
- sar-H
- sar-n-DEV
- sar-n-EDEV
- sar-r
- sar-r-ALL
- sar-u (default)
- sar-u-ALL 
- mpstat-P-ALL
- cpu-all (custom option showing CPU time instead of percentages)
- per-cpu-all (custom option showing CPU time instead of percentages per CPU)
- psi-cpu (custom option showing cpu pressure stall information)
- psi-mem (custom option showing memory pressure stall information)
- psi-io (custom option showing io pressure stall information)

## The webserver
Currently, `procstat` always starts a webserver on port `1111`. I am considering enabling and disabling this via a switch, and the port should be configurable in the future.

The webserver allows to see graphs of CPU, memory, disk IO and networking.
Examples:

CPU usage:
![CPU](/doc/cpu-load-psi.png)
Memory usage:
![Memory](/doc/memory.png)
System wide disk IO:
![Disk IO](/doc/blockdevices.png)
System wide network IO:
![Network IO](/doc/networkdevices.png)

## Warning
This is a preview version. Feedback is appreciated, as well as any issues that are encountered.

# Building `procstat`
Please mind the building steps are validated for linux and MacOS.
`procstat` requires the linux operating system and access to the `/proc` and `/sys` filesystems to perform its function.

Basic steps:
- Install the rust suite: see <https://www.rust-lang.org/tools/install>
- Clone the this repository: `git clone https://github.com/FritsHoogland/procstat.git`
- Build the executable:
```
cd procstat
cargo build --release
```
The executable is available at `./target/release/procstat` after compilation (cargo build).
The executable can also be run using cargo: `cargo run --release`. Please mind any switches must be set *after* adding `--` to the cargo run command.

