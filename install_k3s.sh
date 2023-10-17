#!/bin/bash


export DATASTORE="postgresql://linpostgres:.VhbDwmE5j1tioL9@tcp(lin-7198-1350-pgsql-primary-private.servers.linodedb.net:5432)/k3s"
export TOKEN=6ae21386ad2937e75228de6a5b45eb48764ae128

k3sup install --host 192.168.1.247 --ssh-key ~/.ssh/id_ed25519 --user freexploit  --local-path ~/.kube/k3s_pi.yaml --context cluster_pi --cluster --k3s-extra-args "--tls-san 100.96.174.126/32 --disable=traefik" --token="${TOKEN} "

