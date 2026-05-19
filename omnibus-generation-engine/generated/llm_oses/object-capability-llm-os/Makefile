ARCH ?= x86_64
TARGET = object_capability_llm_os

.PHONY: all clean run

all:
	@echo "Build object-capability-llm-os for $(ARCH)"
	@echo "Languages: rust, javascript"
	@echo "Kernel type: object-capability"
	@echo "LLM subsystem: delegated_generation"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
