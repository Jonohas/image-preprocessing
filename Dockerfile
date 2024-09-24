FROM rust:1.80

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get -y update
RUN apt-get -y upgrade

RUN cargo install --path .

CMD ["image_preprocessing"]
