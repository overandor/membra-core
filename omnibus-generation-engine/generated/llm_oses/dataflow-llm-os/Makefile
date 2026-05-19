ARCH ?= x86_64
TARGET = dataflow_llm_os

.PHONY: all clean run

all:
	@echo "Build dataflow-llm-os for $(ARCH)"
	@echo "Languages: rust, cpp, c"
	@echo "Kernel type: dataflow"
	@echo "LLM subsystem: tensor_graph"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
