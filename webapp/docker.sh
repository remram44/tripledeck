#!/bin/sh
docker run -ti --rm -v $PWD:/usr/src/app -w /usr/src/app -p 8080:8080 node:8 sh -c 'npm install && npm run serve -- --host 0.0.0.0'
