ARCH ?= x86_64
TARGET = mesh_llm_os

.PHONY: all clean run

all:
	@echo "Build mesh-llm-os for $(ARCH)"
	@echo "Languages: rust, c"
	@echo "Kernel type: mesh"
	@echo "LLM subsystem: nearest_gpu_router"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
