FROM solr:8.11.1

COPY security.json security.json
COPY solr-security.sh /usr/bin/solr-security.sh
ENTRYPOINT ["/usr/bin/solr-security.sh"]