prog :=html-entities

debug ?=

ifdef debug
	release :=
	target :=debug
else
	release :=--release
	target :=release
endif

build:
	cargo build $(release)

install:
ifdef debug
	cp target/$(target)/$(prog) /usr/bin/$(prog)-debug
	chmod 755 /usr/bin/$(prog)-debug
else
	cp target/$(target)/$(prog) /usr/bin/$(prog)
	chmod 755 /usr/bin/$(prog)
endif

cpfile:
	mkdir -p /usr/share/html-entities
	cp -f ./resources/html-entities.json /usr/share/html-entities/

all: build install cpfile

uninstall:
ifdef debug
	rm /usr/bin/$(prog)-debug
else
	rm /usr/bin/$(prog)
endif
	rm -rf /usr/share/html-entities/

help:
	@echo "usage: make $(prog) [debug=1]"