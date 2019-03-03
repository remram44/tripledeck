#!/bin/sh
if [ "$#" != 1 ]; then
    echo "install|build|serve" >&2
    exit 1
elif [ "$1" = install ]; then
    docker run -i --rm -v $PWD:/usr/src/app -w /usr/src/app -p 8080:8080 node:8 npm install
elif [ "$1" = serve ]; then
    docker run -i --rm -v $PWD:/usr/src/app -w /usr/src/app -p 8080:8080 node:8 npm run serve -- --host 0.0.0.0
elif [ "$1" = build ]; then
    docker run -i --rm -v $PWD:/usr/src/app -w /usr/src/app -p 8080:8080 node:8 node_modules/webpack/bin/webpack.js
fi
