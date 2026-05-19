ARCH ?= x86_64
TARGET = deterministic_llm_os

.PHONY: all clean run

all:
	@echo "Build deterministic-llm-os for $(ARCH)"
	@echo "Languages: rust, c"
	@echo "Kernel type: deterministic"
	@echo "LLM subsystem: reproducible_engine"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
