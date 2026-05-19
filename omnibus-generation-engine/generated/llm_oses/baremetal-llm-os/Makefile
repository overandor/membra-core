ARCH ?= x86_64
TARGET = baremetal_llm_os

.PHONY: all clean run

all:
	@echo "Build baremetal-llm-os for $(ARCH)"
	@echo "Languages: c, rust, assembly"
	@echo "Kernel type: baremetal"
	@echo "LLM subsystem: direct_dma"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
