docker exec -it $(docker ps --filter "ancestor=procstat" -q) bash
