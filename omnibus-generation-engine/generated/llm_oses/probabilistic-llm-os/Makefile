ARCH ?= x86_64
TARGET = probabilistic_llm_os

.PHONY: all clean run

all:
	@echo "Build probabilistic-llm-os for $(ARCH)"
	@echo "Languages: rust, python, c"
	@echo "Kernel type: probabilistic"
	@echo "LLM subsystem: entropy_scheduler"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
