FROM rustlang/rust:nightly

WORKDIR /usr/src/onebit
COPY . .

RUN cp program/encode.bin /usr/share/

WORKDIR vm
RUN cargo install --path .

CMD ["vm"]