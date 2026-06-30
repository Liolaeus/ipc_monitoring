SUMMARY = "IPC monitoring project"
# LICENSE = "MIT"
# LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"


SRC_URI = "git://[github.com/Liolaeus/ipc_monitoring.git;protocol=https;branch=master](https://github.com/Liolaeus/ipc_monitoring.git;protocol=https;branch=master) \
           file://broker.service \
           file://sensor.service \
           file://sensor.conf \
           file://processor.service \
           file://processor.conf \
           file://alerter.service \
           file://alerter.conf"

SRCREV = "${AUTOREV}"
S = "${WORKDIR}/git"

# 2. Inherit both cargo and systemd
inherit cargo systemd

# 3. Define ALL systemd services to be enabled on boot
SYSTEMD_SERVICE:${PN} = "broker.service sensor.service processor.service alerter.service"
SYSTEMD_AUTO_ENABLE = "enable"

# 4. Install the service and config files
# Note: The cargo bbclass automatically installs the binaries into ${D}${bindir} (/usr/bin),
# so we only need to manually install the systemd and /etc files here.
do_install:append() {
    # Install Systemd Units
    install -d ${D}${systemd_system_unitdir}
    install -m 0644 ${WORKDIR}/broker.service ${D}${systemd_system_unitdir}
    install -m 0644 ${WORKDIR}/sensor.service ${D}${systemd_system_unitdir}
    install -m 0644 ${WORKDIR}/processor.service ${D}${systemd_system_unitdir}
    install -m 0644 ${WORKDIR}/alerter.service ${D}${systemd_system_unitdir}

    # Install Environment Configs
    install -d ${D}${sysconfdir}/default
    install -m 0644 ${WORKDIR}/sensor.conf ${D}${sysconfdir}/default/sensor
    install -m 0644 ${WORKDIR}/processor.conf ${D}${sysconfdir}/default/processor
    install -m 0644 ${WORKDIR}/alerter.conf ${D}${sysconfdir}/default/alerter
}

# 5. Tell Yocto to package the new /etc/ files into the final image
FILES:${PN} += "${sysconfdir}/default/ipc_monitoring-*"
