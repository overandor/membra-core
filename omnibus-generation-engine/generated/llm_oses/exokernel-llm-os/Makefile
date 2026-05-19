ARCH ?= x86_64
TARGET = exokernel_llm_os

.PHONY: all clean run

all:
	@echo "Build exokernel-llm-os for $(ARCH)"
	@echo "Languages: rust, c, cpp"
	@echo "Kernel type: exokernel"
	@echo "LLM subsystem: hardware_multiplexer"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
