#!/bin/sh
solr zk cp /opt/solr-9.3.0/security.json zk:security.json -z zoo1:2181
exec /opt/solr-9.3.0/docker/scripts/docker-entrypoint.sh -f "$@"