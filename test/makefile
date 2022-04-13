CC = clang
CFLAGS += -O0 -g

all: sample sample-cov sample.ll

sample: sample.c
	$(CC) $(CFLAGS) -o $@ $<

sample.o: sample.c
	$(CC) $(CFLAGS) -fsanitize-coverage=trace-pc-guard -c -o $@ $<

sample.ll: sample.c
	$(CC) $(CFLAGS) -fsanitize-coverage=trace-pc-guard -S -emit-llvm -o $@ $<

sample-cov: sample.o
	cd .. && cargo build --release
	$(CC) $(CFLAGS) -o $@ $< ../target/release/libsancov_dumper.a

clean:
	rm -rf sample sample-cov sample.{o,ll}
