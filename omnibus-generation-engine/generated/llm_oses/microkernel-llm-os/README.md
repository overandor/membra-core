# microkernel-llm-os

Microkernel where LLM inference runs as a privileged server with capability-based IPC.

## Attributes
- **Kernel type**: microkernel
- **LLM subsystem**: inference_server
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
