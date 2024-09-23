# sendmyip
Rust client that sends host IP information to server



## Tests
Tests require that SENDMYIP_INTERFACE environment variable is set to the correct network interface. Otherwise default "eth0" is used, which may or may not be correct.

Example:

```bash
export SENDMYIP_INTERFACE=wlp58s0
```
You can find out your correct network interface on Linux by running `ifconfig`

## License
This code is licensed under the GPL v3 license.

