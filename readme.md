# IPC Monitoring

A Rust IPC monitoring stack for simulated sensor data and a Yocto layer for packaging it into an embedded Linux image.

## Overview

The project contains four Rust services:

- broker: runs an MQTT broker (rumqttd) that receives and forwards messages.
- sensor: publishes mocked temperature readings to the broker.
- processor: reads temperature data and prints running statistics: average, minimum, maximum.
- alerter: reads temperature data and reports abnormal value based on configured limits.

## Requirements

- Rust toolchain (tested with Rust 1.93.x)
- Taskfile (task)
- Git

For the Yocto build flow, you also need a standard Yocto build host with common build tools such as:
- Python 3
- git
- the usual packages required by bitbake and Poky

## Repository layout

- `monitoring/`: Rust workspace and service sources
- `yocto/`: Yocto layer and build setup

## Building and running the Rust services

From the `monitoring` directory:

run any of the services:
```bash
task {alerter|broker|processor|sensor}
```

of from any of the services directory:
```bash
task build
# or
task run
```

### Configuration

The services use environment variables with default values:

- `BROKER_IP` and `BROKER_PORT` for the client services (default: `127.0.0.1:1883`)
- `RUMQTTD_CONF` config file path for the broker (default: `rumqttd.toml`)
- `BOUND_LOW` and `BOUND_HIGH` for alerter (default: `-10` and `150`)

## Yocto build flow

From the repository root:
```bash
# clone the bitbake repo to setup poky build env
git clone https://git.openembedded.org/bitbake

# setup to use poky-wrynose, and qemux86-64
./bitbake/bin/bitbake-setup init

# setup build env
cd bitbake-builds/poky-wrynose
source build/init-build-env

# initialize a layer
bitbake-layers create-layer ../ipc-monitoring
bitbake-layers add-layer ../ipc-monitoring

# while in build, copy conf and layer files
cp ../../../ipc_monitoring_layer/conf/layer.conf ../ipc-monitoring/conf/layer.conf
cp ../../../ipc_monitoring_layer/conf/local.conf conf/local.conf

cp -r ../../../ipc_monitoring_layer/recipes-ipc ../ipc-monitoring/recipes-ipc
cp ../../../../rumqttd.toml ../ipc-monitoring/recipes-ipc/images/files/

# build the image with our layer
bitbake core-image-minimal

# run in qemu
runqemu snapshot
```

Once the QEMU image is running, you can inspect the services with:

```bash
# login with: root, no password
systemctl status broker
systemctl status sensor
systemctl status processor
systemctl status alerter
```
