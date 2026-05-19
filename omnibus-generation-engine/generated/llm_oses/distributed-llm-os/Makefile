ARCH ?= x86_64
TARGET = distributed_llm_os

.PHONY: all clean run

all:
	@echo "Build distributed-llm-os for $(ARCH)"
	@echo "Languages: rust, go, c"
	@echo "Kernel type: distributed"
	@echo "LLM subsystem: process_migrator"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
