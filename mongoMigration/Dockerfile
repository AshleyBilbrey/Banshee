FROM node:latest

RUN npm install mongodb pg

COPY index.js index.js

ENTRYPOINT [ "node", "index.js" ]
