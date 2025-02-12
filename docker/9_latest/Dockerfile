FROM solr:9.8

COPY security.json security.json
COPY solr-security.sh /usr/bin/solr-security.sh
ENTRYPOINT ["/usr/bin/solr-security.sh"]