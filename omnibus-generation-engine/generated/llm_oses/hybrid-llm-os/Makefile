ARCH ?= x86_64
TARGET = hybrid_llm_os

.PHONY: all clean run

all:
	@echo "Build hybrid-llm-os for $(ARCH)"
	@echo "Languages: rust, c, cpp"
	@echo "Kernel type: hybrid"
	@echo "LLM subsystem: cost_router"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
