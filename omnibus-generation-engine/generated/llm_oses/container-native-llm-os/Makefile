ARCH ?= x86_64
TARGET = container_native_llm_os

.PHONY: all clean run

all:
	@echo "Build container-native-llm-os for $(ARCH)"
	@echo "Languages: rust, go, c"
	@echo "Kernel type: container-native"
	@echo "LLM subsystem: inference_container"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
