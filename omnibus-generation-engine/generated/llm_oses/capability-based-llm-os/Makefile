ARCH ?= x86_64
TARGET = capability_based_llm_os

.PHONY: all clean run

all:
	@echo "Build capability-based-llm-os for $(ARCH)"
	@echo "Languages: rust, c"
	@echo "Kernel type: capability"
	@echo "LLM subsystem: capability_gpu"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
