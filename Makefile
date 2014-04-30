all: bin src

.PHONY: bin
bin: bin/Makefile
	cd bin; $(MAKE); cd ..

.PHONY: src
src: src/Makefile
	cd src; $(MAKE); cd ..
