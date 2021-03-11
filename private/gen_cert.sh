#! /bin/bash

CA_SUBJECT="/C=US/ST=CA/O=Sailex CA/CN=Sailex Root CA"
SUBJECT="/C=US/ST=CA/O=Sailex/CN=localhost"
ALT="DNS:localhost"

openssl genrsa -out ca_key.pem 4096
openssl req -new -x509 -days 3650 -key ca_key.pem -subj "${CA_SUBJECT}" -out ca_cert.pem

openssl req -newkey rsa:4096 -nodes -sha256 -keyout key.pem -subj "${SUBJECT}" -out server.csr
openssl x509 -req -sha256 -extfile <(printf "subjectAltName=${ALT}") -days 3650 \
    -CA ca_cert.pem -CAkey ca_key.pem -CAcreateserial \
    -in server.csr -out cert.pem

rm ca_cert.srl server.csr