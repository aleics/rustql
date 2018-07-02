FROM rust:1.26.0

# set working directory and copy content
WORKDIR /usr/src/rustql
COPY . .

# install rustql in container
RUN cargo install

CMD ["rustql"]