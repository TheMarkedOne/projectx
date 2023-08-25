FROM node:18.16

WORKDIR /usr/src/app

COPY package*.json .

RUN npm install

RUN npm update

RUN npm install -g @nestjs/cli

COPY . .

CMD ["npm", "run", "start:dev"]
