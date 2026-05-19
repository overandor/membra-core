ARCH ?= x86_64
TARGET = symbiotic_llm_os

.PHONY: all clean run

all:
	@echo "Build symbiotic-llm-os for $(ARCH)"
	@echo "Languages: rust, c, haskell"
	@echo "Kernel type: symbiotic"
	@echo "LLM subsystem: fair_arbiter"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
