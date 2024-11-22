# Missing USDT probes repro

## Reproducing

1. Build it:

```
cargo build --release
```

2. Run bpftrace to catch `cache:hit`:

```
$ sudo bpftrace -e 'usdt:./target/release/probe-derp:cache:hit { printf("hit: %s\n", str(arg0)) }'
Attaching 3 probes...
```

3. Run the compiled program:

```
$ ./target/release/probe-derp
first value (miss) : whoa: Key { inner: "beep" }
first value (hit)  : whoa: Key { inner: "beep" }
```

4. Observe no output in `bpftrace` (bad!).

## Notes

It does not reproduce if you make the program synchronous:

* Remove `#[tokio::main]`
* Replace `async fn main()` -> `fn main()`
* Remove any `.await` and `async`

```
$ sudo bpftrace -e 'usdt:./target/release/probe-derp:cache:hit { printf("hit: %s\n", str(arg0)) }'
Attaching 1 probe...
hit: Key { inner: "beep" }
```

Note that there's only 1 probe compared to 3 before.

Before:

```
$ readelf -n ./target/release/probe-derp

Displaying notes found in: .note.gnu.property
  Owner                Data size 	Description
  GNU                  0x00000010	NT_GNU_PROPERTY_TYPE_0
      Properties: x86 ISA needed: x86-64-baseline

Displaying notes found in: .note.gnu.build-id
  Owner                Data size 	Description
  GNU                  0x00000014	NT_GNU_BUILD_ID (unique build ID bitstring)
    Build ID: 463a9edbd15a19102036eb5377e955dbe2ab0833

Displaying notes found in: .note.ABI-tag
  Owner                Data size 	Description
  GNU                  0x00000010	NT_GNU_ABI_TAG (ABI version tag)
    OS: Linux, ABI: 3.2.0

Displaying notes found in: .note.stapsdt
  Owner                Data size 	Description
  stapsdt              0x0000002a	NT_STAPSDT (SystemTap probe descriptors)
    Provider: cache
    Name: hit
    Location: 0x000000000000c059, Base: 0x000000000007724c, Semaphore: 0x000000000009007c
    Arguments: -8@%r15
  stapsdt              0x0000002b	NT_STAPSDT (SystemTap probe descriptors)
    Provider: cache
    Name: miss
    Location: 0x000000000000c121, Base: 0x000000000007724c, Semaphore: 0x000000000009007e
    Arguments: -8@%r15
  stapsdt              0x0000002a	NT_STAPSDT (SystemTap probe descriptors)
    Provider: cache
    Name: hit
    Location: 0x000000000000efae, Base: 0x000000000007724c, Semaphore: 0x000000000009007c
    Arguments: -8@%rbx
  stapsdt              0x0000002b	NT_STAPSDT (SystemTap probe descriptors)
    Provider: cache
    Name: miss
    Location: 0x000000000000f01d, Base: 0x000000000007724c, Semaphore: 0x000000000009007e
    Arguments: -8@%rbx
  stapsdt              0x0000002a	NT_STAPSDT (SystemTap probe descriptors)
    Provider: cache
    Name: hit
    Location: 0x0000000000011aa3, Base: 0x000000000007724c, Semaphore: 0x000000000009007a
    Arguments: -8@%r15
```

After:

```
$ readelf -n ./target/release/probe-derp

Displaying notes found in: .note.gnu.property
  Owner                Data size 	Description
  GNU                  0x00000010	NT_GNU_PROPERTY_TYPE_0
      Properties: x86 ISA needed: x86-64-baseline

Displaying notes found in: .note.gnu.build-id
  Owner                Data size 	Description
  GNU                  0x00000014	NT_GNU_BUILD_ID (unique build ID bitstring)
    Build ID: 9f0f8d336444bb66182d4ee46a8971d5eb2ccf0a

Displaying notes found in: .note.ABI-tag
  Owner                Data size 	Description
  GNU                  0x00000010	NT_GNU_ABI_TAG (ABI version tag)
    OS: Linux, ABI: 3.2.0

Displaying notes found in: .note.stapsdt
  Owner                Data size 	Description
  stapsdt              0x0000002b	NT_STAPSDT (SystemTap probe descriptors)
    Provider: cache
    Name: miss
    Location: 0x00000000000086bd, Base: 0x0000000000051428, Semaphore: 0x000000000005e062
    Arguments: -8@%r15
  stapsdt              0x0000002a	NT_STAPSDT (SystemTap probe descriptors)
    Provider: cache
    Name: hit
    Location: 0x0000000000008a86, Base: 0x0000000000051428, Semaphore: 0x000000000005e060
    Arguments: -8@%r12
```
