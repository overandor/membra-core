ARCH ?= x86_64
TARGET = reactive_llm_os

.PHONY: all clean run

all:
	@echo "Build reactive-llm-os for $(ARCH)"
	@echo "Languages: rust, cpp"
	@echo "Kernel type: reactive"
	@echo "LLM subsystem: observable_output"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
