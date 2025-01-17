# HTTPSShell

Tested docker build in kali.

Setup:
----
1. Run docker build in directory with Dockerfile:
```
docker build .
```
2. Run container (use -v flag to mount directories as desired):
```
docker run -di [imageID]
```
3. Enter container:
```
docker exec -ti [containerID] /bin/bash
```
4. From within container, Clone repo:
```
git clone https://github.com/Business1sg00d/RustHTTPSShell.git /opt/RustHTTPSShell
cd /opt/RustHTTPSShell
```
5. From within container, add windows support:
```
rustup target add x86_64-pc-windows-gnu
```

Building from within container:
-----
When building for windows use:
```
cargo build --target x86_64-pc-windows-gnu --bin main_windows
```

For linux use:
```
cargo build
```

Binaries are in "target" directory from within project directory.
