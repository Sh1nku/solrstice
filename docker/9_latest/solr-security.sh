#!/bin/sh
solr zk cp security.json zk:security.json -z zoo1:2181
exec docker/scripts/docker-entrypoint.sh -f "$@"