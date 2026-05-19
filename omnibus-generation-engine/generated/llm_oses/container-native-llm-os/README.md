# container-native-llm-os

Kernel is a container orchestrator; every LLM inference is a sandboxed container with cgroup v3.

## Attributes
- **Kernel type**: container-native
- **LLM subsystem**: inference_container
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
