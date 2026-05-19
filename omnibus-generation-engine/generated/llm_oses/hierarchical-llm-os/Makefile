ARCH ?= x86_64
TARGET = hierarchical_llm_os

.PHONY: all clean run

all:
	@echo "Build hierarchical-llm-os for $(ARCH)"
	@echo "Languages: rust, c, cpp"
	@echo "Kernel type: hierarchical"
	@echo "LLM subsystem: tiered_model"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
