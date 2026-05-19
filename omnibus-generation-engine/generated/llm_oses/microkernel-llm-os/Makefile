ARCH ?= x86_64
TARGET = microkernel_llm_os

.PHONY: all clean run

all:
	@echo "Build microkernel-llm-os for $(ARCH)"
	@echo "Languages: rust, c"
	@echo "Kernel type: microkernel"
	@echo "LLM subsystem: inference_server"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
