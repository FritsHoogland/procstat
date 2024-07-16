docker build -f docker/Dockerfile.build_ubuntu_2204 --tag procstat_ubuntu_2204 --target export-stage --output type=local,dest=. .
