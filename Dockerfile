FROM alpine:3.15.0

CMD [ "/bin/log-converter" ]

COPY /target/release/log-converter /bin/
