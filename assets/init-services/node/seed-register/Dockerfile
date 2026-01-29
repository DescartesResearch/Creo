FROM node:22.13.1-bookworm-slim

RUN mkdir -p /home/node/app/node_modules && chown -R node:node /home/node/app

WORKDIR /home/node/app

COPY package.json ./

RUN npm install

USER node

COPY --chown=node:node src src

ENTRYPOINT ["node", "src/index.js"]
