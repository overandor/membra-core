# mesh-llm-os

Mesh-topology OS for edge clusters; inference workloads route through nearest GPU hop.

## Attributes
- **Kernel type**: mesh
- **LLM subsystem**: nearest_gpu_router
- **Languages**: rust, c

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
