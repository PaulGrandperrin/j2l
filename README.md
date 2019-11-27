# Usage
Print formatted logs without stack traces:

`stern <COMMAND PARAMS> | j2l`

Print formatted logs with stack traces:

`stern <COMMAND PARAMS> | j2l -s`

# Installation
Run the installer:

`sudo ./build.sh`

Add symbolic link to program:

`sudo ln -s <PATH_ON_DISK>/j2l/target/<YOUR_SYSTEM_ARCHITECTURE>/release/j2l /usr/local/bin/j2l`