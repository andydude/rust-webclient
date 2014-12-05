all: libwebclient.stamp dgst

RUSTC = rustc
RUSTARGS=-O -L .

libwebclient_SOURCES = \
	libwebclient/bits/mod.rs \
	libwebclient/bits/u32.rs \
	libwebclient/bits/u64.rs \
	libwebclient/digest/md5.rs \
	libwebclient/digest/mod.rs \
	libwebclient/digest/sha1.rs \
	libwebclient/digest/sha2.rs \
	libwebclient/digest/sha3.rs \
	libwebclient/lib.rs

libwebclient.stamp: $(libwebclient_SOURCES)
	$(RUSTC) $(RUSTARGS) libwebclient/lib.rs && touch $@

dgst: dgst.rs
	$(RUSTC) $(RUSTARGS) dgst.rs

test:
	$(RUSTC) $(RUSTARGS) --test libwebclient/lib.rs

clean:
	rm -f libwebclient-*.dylib
	rm -f libwebclient-*.rlib
	rm -f libwebclient.stamp
	rm -f dgst
