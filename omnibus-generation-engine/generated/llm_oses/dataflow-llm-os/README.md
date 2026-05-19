# dataflow-llm-os

OS structured as a dataflow graph; LLM layers are nodes, tensors flow on edges.

## Attributes
- **Kernel type**: dataflow
- **LLM subsystem**: tensor_graph
- **Languages**: rust, cpp, c

## Architecture

```
bootloader/   → minimal stub or GRUB multiboot
kernel/       → kernel main, scheduler, memory manager
llm_subsystem/→ token inference, KV-cache, model loader
drivers/      → UART, framebuffer, NVMe, GPU
```

## Build

```bash
make
```

## Run in QEMU

```bash
make run ARCH=x86_64
```

## License
MIT
