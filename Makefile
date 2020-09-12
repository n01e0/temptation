SRC=sample.c

all:
	gcc $(SRC) -o fileless_sample
	cargo build

test: fileless_sample
	LD_PRELOAD=./target/debug/libtemptation.so ./fileless_sample

clean: fileless_sample
	rm fileless_sample

