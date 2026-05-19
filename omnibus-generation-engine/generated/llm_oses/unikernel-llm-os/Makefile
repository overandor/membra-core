ARCH ?= x86_64
TARGET = unikernel_llm_os

.PHONY: all clean run

all:
	@echo "Build unikernel-llm-os for $(ARCH)"
	@echo "Languages: rust, c"
	@echo "Kernel type: unikernel"
	@echo "LLM subsystem: monolithic_inference"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
