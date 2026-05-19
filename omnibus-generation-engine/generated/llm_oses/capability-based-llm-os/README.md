# capability-based-llm-os

Capability system where file handles, GPU contexts, and model weights are all capabilities.

## Attributes
- **Kernel type**: capability
- **LLM subsystem**: capability_gpu
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
