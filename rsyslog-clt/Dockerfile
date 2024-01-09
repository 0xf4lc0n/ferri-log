FROM arm64v8/ubuntu:noble

RUN apt update -y
RUN apt install -y rsyslog
RUN apt install -y vim

RUN echo 'module(load="imtcp")' >> /etc/rsyslog.conf
RUN echo 'input(type="imtcp" port="514")' >> /etc/rsyslog.conf
RUN echo '*.* action(type="omfwd" target="ferri-log-server" port="514" protocol="tcp")' >> /etc/rsyslog.conf

CMD [ "/bin/bash", "-c", "rsyslogd; while true; do sleep 1000; done" ]


