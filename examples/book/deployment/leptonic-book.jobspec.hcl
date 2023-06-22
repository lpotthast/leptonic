job "leptonic-book" {
  datacenters = ["dc1"]
  type        = "service"

  group "leptonic-book" {
    count = 1

    network {
      port "http" {}
      port "https" {}
    }

    restart {
      attempts = 10
      interval = "5m"
      delay    = "25s"
      mode     = "delay"
    }

    task "leptonic-book" {
      driver = "docker"

      env {
        DOMAIN = "leptonic.dev"
        CONSUL_HTTP_ADDR = "172.17.0.1:8500"
        HTTP_PORT = "${NOMAD_PORT_http}"
        HTTPS_PORT = "${NOMAD_PORT_https}"
        URL_PREFIX = "/leptonic"
      }

      config {
        image = "registry.gitlab.com/lukaspotthast/leptonic/leptonic-book:{{TAG}}"
        auth {
          username = "{{REGISTRY_USERNAME}}"
          password = "{{REGISTRY_TOKEN}}"
        }
        ports = ["http", "https"]
      }

      resources {
        cpu    = 500
        memory = 256
      }

      service {
        name = "leptonic-book"
        tags = [
          "urlprefix-leptonic.dev:80/ redirect=301,https://leptonic.dev/",
          "urlprefix-/ redirect=301,/leptonic/",
          "urlprefix-${URL_PREFIX} proto=https tlsskipverify=true"
        ]
        port = "https"

        check {
          type     = "tcp"
          interval = "10s"
          timeout  = "2s"
        }
      }
    }
  }
}
