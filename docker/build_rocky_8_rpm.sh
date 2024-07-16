docker build -f docker/Dockerfile.build_rocky_8 --tag procstat_rocky_8 --target export-stage --output type=local,dest=. .
