FROM rustlang/rust:nightly

# set working directory and copy content
WORKDIR /usr/src/rustql
COPY . .

# install rustql in container
RUN cargo install --path .

CMD ["rustql"]