FROM lukemathwalker/cargo-chef:latest-rust-1.75.0 AS chef
WORKDIR /app

FROM chef AS planner

COPY Cargo.lock .
COPY Cargo.toml .       
COPY application application       
COPY domain domain
COPY infrastructure infrastructure
COPY migrations migrations
COPY server server
COPY web_app web_app

RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.lock .
COPY Cargo.toml .       
COPY application application       
COPY domain domain
COPY infrastructure infrastructure
COPY migrations migrations
COPY server server
COPY web_app web_app

RUN cargo build --release --bin server

RUN mkdir /ferri-log

RUN cp target/release/server /ferri-log/server
RUN cp -r server/configuration /ferri-log/configuration
RUN cp -r server/certs /ferri-log/certs

FROM debian:bookworm AS runtime
WORKDIR /app

RUN apt update -y
RUN apt-get -y install libssl-dev
RUN apt install -y rsyslog
RUN apt install -y vim
RUN echo 'module(load="imtcp")' >> /etc/rsyslog.conf
RUN echo 'input(type="imtcp" port="514")' >> /etc/rsyslog.conf
RUN echo 'template(name="CustomJsonFmt" type="list" option.jsonf="on") {\
    property(outname="timestamp" name="timereported" dateFormat="rfc3339" format="jsonf")\
    property(outname="host" name="hostname" format="jsonf")\
    property(outname="severity" name="syslogseverity-text" caseConversion="upper" format="jsonf")\
    property(outname="facility" name="syslogfacility-text" format="jsonf")\
    property(outname="syslog_tag" name="syslogtag" format="jsonf")\
    property(outname="source" name="app-name" format="jsonf" onEmpty="null")\
    property(outname="message" name="msg" format="jsonf")}' >> /etc/rsyslog.conf
RUN echo 'template(name="CustomJsonFmtFile" type="string" string="/var/log/remote/%HOSTNAME%/%PROGRAMNAME%.log")' >> /etc/rsyslog.conf
RUN echo 'action(type="omfile" dynaFile="CustomJsonFmtFile" template="CustomJsonFmt" dirGroup="adm" fileGroup="adm" fileCreateMode="0640" dirCreateMode="0755")' >> /etc/rsyslog.conf
RUN sed -i '/^module(load="imklog")/ s/^/#/' /etc/rsyslog.conf

COPY --from=builder /ferri-log .
ENV RUST_LOG=server=info,error
ENTRYPOINT  [ "/app/server" ] 