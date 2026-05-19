ARCH ?= x86_64
TARGET = cloud_llm_os

.PHONY: all clean run

all:
	@echo "Build cloud-llm-os for $(ARCH)"
	@echo "Languages: rust, go, c"
	@echo "Kernel type: cloud"
	@echo "LLM subsystem: gpu_timeslicer"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
