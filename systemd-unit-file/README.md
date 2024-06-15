# Procstat systemd unit file

A systemd unit file is the startup and shutdown script for systemd managed servers.

Do carefully inspect the unit file, and only proceed if you understand the settings.
The most prominent setting is the working directory (WorkingDirectory), which is where procstat will generate its archives.

# Installation

1. Copy the unit/service file to `/etc/systemd/system/`.
2. Reload systemd: `systemctlt daemon-reload` (this is safe and does not interrupt any processes).
3. Set the unit to be started at boot: `systemctl enable procstat`. (if you want to only run procstat at specific times, do not run "enable")
4. Start the unit: `systemctl start procstat`.

# Warning!

Procstat currently does not clean up it's archive files, so that is a task that has to be executed independently from procstat.

# Removal

1. Stop the service if running: `systemctl stop procstat`.
2. Remove the unit: `systemctl disable procstat`.
3. Remove the unit file: `rm /etc/systemd/system/procstat.service`.
4. Reload systemd: `systemctl daemon-reload`.
