all: sample

sample: sample.c
	gcc sample.c -o fileless_sample
	cd ../ && cargo build --release

test: fileless_sample ../target/debug/libtemptation.so
	@echo "[+] Default executing..."
	./fileless_sample
	@echo "[+] Hooking fileless exec..."
	LD_PRELOAD=../target/release/libtemptation.so ACTION=dump ./fileless_sample

clean:
	rm *.dump fileless_sample
