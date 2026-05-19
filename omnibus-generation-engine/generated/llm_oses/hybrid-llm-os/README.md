# hybrid-llm-os

Seamlessly offloads between on-device NPU, edge GPU, and cloud TPU based on latency cost.

## Attributes
- **Kernel type**: hybrid
- **LLM subsystem**: cost_router
- **Languages**: rust, c, cpp

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
