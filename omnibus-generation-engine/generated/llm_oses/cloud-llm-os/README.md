# cloud-llm-os

Multi-tenant kernel with GPU time-slicing and spot-preemptible LLM inference jobs.

## Attributes
- **Kernel type**: cloud
- **LLM subsystem**: gpu_timeslicer
- **Languages**: rust, go, c

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
