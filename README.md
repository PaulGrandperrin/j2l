# Usage
Print formatted logs without stack traces:

`stern <COMMAND PARAMS> | j2l`

Print formatted logs with stack traces:

`stern <COMMAND PARAMS> | j2l -s`

Show the available options

`j2l -h`

# Installation
Run the installer:

`./build.sh`

Add symbolic link to program:

`sudo ln -s target/*/release/j2l /usr/local/bin/`

# Docker

https://hub.docker.com/repository/docker/paulgrandperrin/j2l
