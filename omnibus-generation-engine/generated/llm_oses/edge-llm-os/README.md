# edge-llm-os

Gateway OS caching model shards and serving them to low-power clients over LoRa/WiFi.

## Attributes
- **Kernel type**: edge
- **LLM subsystem**: shard_cache
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
