ARCH ?= x86_64
TARGET = serverless_llm_os

.PHONY: all clean run

all:
	@echo "Build serverless-llm-os for $(ARCH)"
	@echo "Languages: rust, go, c"
	@echo "Kernel type: serverless"
	@echo "LLM subsystem: hot_weight_cache"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
