# axum_todo
this project is my axum entry project on lima/docker.

## installation
1. Clone the repository.
2. `cd axum_todo`
3. `mv .env_template .env`
4. `vi .env` and edit your configure
```
USER_ID=$(id -u) # get your id on the command and replace right of formula.
GID=$(id -g)     # get group id on the command and replace right of formula.
UNAME=your_container_user_name
APP_EARTH_PATH=/home/your_container_user_name/apps
# Receives requests from http://localhost:$HOST_PORT and
# forwards them to the app on the container.
HOST_PORT=your_host_port_number
# the same value localPort of lima configure when using connect DOCKER_HOST=ssh://....
SSH_PORT=your_docker_ssh_port_number
IMG_NAME=your_docker_image_name       # for my shell script
C_NAME=your_docker_container_name     # for my shell script
ENV_NAME=lima-instance_name           # same lima config file name.
```
5. `vi lima-instance_name.yaml` and edit your lima configure to this project.
```
# vm max capacity default is 100 GiB
disk: "20GiB"
# puts your project path of the host * without $HOME/* by lima's rule (I'm not sure: https://github.com/lima-vm/lima/issues/298)
mounts:
- location: "/path/to/your/project"
  writable: true
# if set DOCKER_HOST=ssh://..:localPort then needs to unmovable ssh port number by the below. check by limactl list.
ssh:
  localPort: 22222
```
6. start lima instance and copy 'unix:///~/docker.sock' *you possibility rename lima instance name if needed before below.
   - first: `limactl start lima-instance_name.yaml`
   - second or later: `limactl start lima-instance_name`
7. `vi code.sh` and paste the value copied from above.
```
export DOCKER_HOST=unix:///...
```
8. `./code.sh` after `chmod u+x ./code.sh`
  - check `echo "$DOCKER_HOST"` equals the set value above on the terminal
    - if unexpected, check and remove grep DOCKER_HOST `~/.bashrc, ~/.bash_profile, ~/.profile, ..`. these are used as the highest priority value. in my recognition.
9. install Dev Containers extension on vscode.

## Usage
### on develop
1. cmd + shift + p >Dev Containers: Reopen in Container
   - if unexpected behavior, install devcontainer cli and build on project root to check the problem.
     - how to install: https://code.visualstudio.com/docs/devcontainers/devcontainer-cli#_installation
2. edit code
3. build (and run) * host port forwarding not working.
   - `Run and Debug` on left panel > `Cargo launch` > type `F5`
   - `cargo build` on terminal
### check app
1. cmd + shift + p >Dev Containers: Reopen in Container
2. `Run and Debug` on left panel > `Cargo Release Build`
3. vscode window's bottom left Click >`Close Remote Connection`
4. reopen project folder
5. click explorer on left panel
6. right click on `docker-compose.prod.yml` > up
7. curl http://localhost:${HOST_PORT} on terminal or your browser. 