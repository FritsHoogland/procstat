# please mind centos 7 is outdated, and doesnt' currently work.
# it might be because my cpu architecture is aarch64, for which no centos 7 sources exist.
docker build -f docker/Dockerfile.build_centos_7 --tag procstat_centos_7 .
