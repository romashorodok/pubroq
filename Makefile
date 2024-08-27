
.PHONY: build
build:
	cargo run -p builder

.PHONY: wasm
wasm:
	cargo run -p builder-wasm

.PHONY: cert
cert:
	openssl req -newkey rsa:2048 -nodes -keyout certificate.key -x509 -out certificate.pem -subj '/CN=Test Certificate' -addext "subjectAltName = DNS:localhost"

.PHONY: fingerprint
fingerprint:
	openssl x509 -pubkey -noout -in certificate.pem | openssl rsa -pubin -outform der | openssl dgst -sha256 -binary | base64

.PHONY: chrome
chrome:
	 chromium --origin-to-force-quic-on=localhost:4433  --ignore-certificate-errors-spki-list=fOn0iRRpEo4XTRIlwnEvdesFYpisMqgeGZ+3HSYzero=
