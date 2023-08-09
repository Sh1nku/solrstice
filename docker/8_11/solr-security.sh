#!/bin/sh
solr zk cp /opt/solr-8.11.1/security.json zk:security.json -z zoo1:2181
exec /opt/docker-solr/scripts/docker-entrypoint.sh -f "$@"