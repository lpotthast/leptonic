FROM registry.gitlab.com/lukaspotthast/docker-images/caddy-consul:latest

COPY {{LEPTONIC_DEV_CERT_PEM_FILE}} /etc/certs/letsencrypt/www.leptonic.dev-cert.pem
COPY {{LEPTONIC_DEV_KEY_PEM_FILE}} /etc/certs/letsencrypt/www.leptonic.dev-key.pem

COPY Caddyfile /etc/caddy/Caddyfile

COPY dist/ /srv
