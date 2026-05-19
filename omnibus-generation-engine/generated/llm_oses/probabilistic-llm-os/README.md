# probabilistic-llm-os

Kernel schedules based on expected information gain; high-entropy prompts get priority.

## Attributes
- **Kernel type**: probabilistic
- **LLM subsystem**: entropy_scheduler
- **Languages**: rust, python, c

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
