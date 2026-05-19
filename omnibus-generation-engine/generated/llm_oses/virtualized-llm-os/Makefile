ARCH ?= x86_64
TARGET = virtualized_llm_os

.PHONY: all clean run

all:
	@echo "Build virtualized-llm-os for $(ARCH)"
	@echo "Languages: rust, c, cpp"
	@echo "Kernel type: virtualized"
	@echo "LLM subsystem: gpu_passthrough"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
