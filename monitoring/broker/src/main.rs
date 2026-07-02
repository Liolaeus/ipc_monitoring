use common::get_env_or;
use rumqttd::{Broker, Config};

fn main() {
    let conf_file = get_env_or("RUMQTTD_CONF", "rumqttd.toml".to_string());

    let cfg = config::Config::builder()
        .add_source(config::File::with_name(&conf_file))
        .build()
        .expect("[Broker] invalid configuration");

    let mqttd_conf: Config = cfg.try_deserialize().unwrap();
    let mut broker = Broker::new(mqttd_conf);
    broker.start().unwrap();
}
