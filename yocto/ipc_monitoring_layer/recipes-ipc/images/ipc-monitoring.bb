SUMMARY = "IPC monitoring project"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"


SRC_URI = "file://broker.service \
           file://broker.conf \
           file://sensor.service \
           file://sensor.conf \
           file://processor.service \
           file://processor.conf \
           file://alerter.service \
           file://alerter.conf \
           file://rumqttd.toml"

EXTERNALSRC = "${THISDIR}/../../../../../../monitoring"
SRCREV = "${AUTOREV}"

# allow cargo to dl deps
CARGO_DISABLE_BITBAKE_VENDORING = "1"
do_compile[network] = "1"
CARGO_BUILD_FLAGS:remove = "--frozen"

inherit cargo systemd externalsrc

SYSTEMD_SERVICE:${PN} = "broker.service sensor.service processor.service alerter.service"
SYSTEMD_AUTO_ENABLE = "enable"

do_install:append() {
    # Install Systemd Units
    install -d ${D}${systemd_system_unitdir}
    install -m 0644 ${THISDIR}/files/broker.service ${D}${systemd_system_unitdir}
    install -m 0644 ${THISDIR}/files/sensor.service ${D}${systemd_system_unitdir}
    install -m 0644 ${THISDIR}/files/processor.service ${D}${systemd_system_unitdir}
    install -m 0644 ${THISDIR}/files/alerter.service ${D}${systemd_system_unitdir}

    # Install Environment Configs
    install -d ${D}${sysconfdir}/default
    install -m 0644 ${THISDIR}/files/broker.conf ${D}${sysconfdir}/default/broker
    install -m 0644 ${THISDIR}/files/sensor.conf ${D}${sysconfdir}/default/sensor
    install -m 0644 ${THISDIR}/files/processor.conf ${D}${sysconfdir}/default/processor
    install -m 0644 ${THISDIR}/files/alerter.conf ${D}${sysconfdir}/default/alerter
    install -m 0644 ${THISDIR}/files/rumqttd.toml ${D}${sysconfdir}/default/rumqttd.toml
}

# package new /etc/ files into the final image
FILES:${PN} += "${sysconfdir}/default/*"
