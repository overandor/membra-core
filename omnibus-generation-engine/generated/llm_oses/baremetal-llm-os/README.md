# baremetal-llm-os

No abstractions; user code directly programs GPU DMA rings and token decode loops.

## Attributes
- **Kernel type**: baremetal
- **LLM subsystem**: direct_dma
- **Languages**: c, rust, assembly

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
