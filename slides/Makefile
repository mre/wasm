.PHONY: all
all: install run

.PHONY: clean
clean:
	rm -rf client_body_temp proxy_temp fastcgi_temp uwsgi_temp scgi_temp

.PHONY: install
install:
	wapm install nginx

.PHONY: run
run: clean
	wapm run nginx -p . -c nginx.conf

.PHONY: miniserve
miniserve:
	miniserve html
