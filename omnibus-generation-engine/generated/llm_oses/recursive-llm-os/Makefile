ARCH ?= x86_64
TARGET = recursive_llm_os

.PHONY: all clean run

all:
	@echo "Build recursive-llm-os for $(ARCH)"
	@echo "Languages: rust, c"
	@echo "Kernel type: recursive"
	@echo "LLM subsystem: speculative_vm"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
