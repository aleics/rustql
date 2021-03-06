FROM rustlang/rust:nightly

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

ENV DB_URL=postgres://postgres@172.11.0.3

# set working directory and copy content
WORKDIR /usr/src/rustql
COPY . .

# install rustql in container
RUN cargo install --path .

EXPOSE 8080

CMD ["rustql"]