ARCH ?= x86_64
TARGET = self_hosting_llm_os

.PHONY: all clean run

all:
	@echo "Build self-hosting-llm-os for $(ARCH)"
	@echo "Languages: rust, c, lisp"
	@echo "Kernel type: self-hosting"
	@echo "LLM subsystem: kernel_codegen"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
