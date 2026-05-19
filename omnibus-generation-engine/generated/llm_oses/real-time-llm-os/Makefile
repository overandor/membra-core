ARCH ?= x86_64
TARGET = real_time_llm_os

.PHONY: all clean run

all:
	@echo "Build real-time-llm-os for $(ARCH)"
	@echo "Languages: c, rust, ada"
	@echo "Kernel type: real-time"
	@echo "LLM subsystem: bounded_inference"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
