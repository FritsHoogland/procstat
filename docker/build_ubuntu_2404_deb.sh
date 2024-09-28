docker build -f docker/Dockerfile.build_ubuntu_2404 --tag procstat_ubuntu_2404 --target export-stage --output type=local,dest=. .
#DOCKER_BUILDKIT=0 docker build -f docker/Dockerfile.build_ubuntu_2404 --tag procstat_ubuntu_2404 .
