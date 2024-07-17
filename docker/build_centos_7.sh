# please mind centos 7 is old.
# centos 7 only exists for x86_64.
# it might be because my cpu architecture is aarch64, for which no centos 7 sources exist.
#DOCKER_BUILDKIT=0 docker build -f docker/Dockerfile.build_centos_7 --tag procstat_centos_7 .
docker build -f docker/Dockerfile.build_centos_7 --tag procstat_centos_7 --target export-stage --output type=local,dest=. .
