ARCH ?= x86_64
TARGET = language_based_llm_os

.PHONY: all clean run

all:
	@echo "Build language-based-llm-os for $(ARCH)"
	@echo "Languages: rust, idris, ats"
	@echo "Kernel type: language-based"
	@echo "LLM subsystem: shape_prover"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
