ARCH ?= x86_64
TARGET = embedded_llm_os

.PHONY: all clean run

all:
	@echo "Build embedded-llm-os for $(ARCH)"
	@echo "Languages: c, rust, zephyr"
	@echo "Kernel type: embedded"
	@echo "LLM subsystem: tinyml_federator"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
