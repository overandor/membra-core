ARCH ?= x86_64
TARGET = flat_address_llm_os

.PHONY: all clean run

all:
	@echo "Build flat-address-llm-os for $(ARCH)"
	@echo "Languages: rust, c"
	@echo "Kernel type: flat-address"
	@echo "LLM subsystem: global_kv_cache"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
