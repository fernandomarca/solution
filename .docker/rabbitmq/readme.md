curl -i -u guest:guest -H "content-type:application/json" \
     -XPUT -d'{"type":"stream"}' \
     <http://localhost:15672/api/queues/%2f/><your-stream-name>
