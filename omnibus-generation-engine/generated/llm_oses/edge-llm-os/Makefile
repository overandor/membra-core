ARCH ?= x86_64
TARGET = edge_llm_os

.PHONY: all clean run

all:
	@echo "Build edge-llm-os for $(ARCH)"
	@echo "Languages: rust, go, c"
	@echo "Kernel type: edge"
	@echo "LLM subsystem: shard_cache"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
