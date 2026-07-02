use std::time::Duration;

use common::{TemperatureData, get_env_or};
use log::{error, info};
use rumqttc::{AsyncClient, ClientError, EventLoop, MqttOptions, QoS};
use tokio::time::sleep;

struct SensorConfig {
    broker_ip: String,
    broker_port: u16,
}

async fn init_event_loop(conf: &SensorConfig) -> Result<(AsyncClient, EventLoop), ClientError> {
    let mut mqtt_options = MqttOptions::new(
        "monitoring-sensor",
        conf.broker_ip.clone(),
        conf.broker_port,
    );
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    Ok(AsyncClient::new(mqtt_options, 0))
}

async fn publish_events(client: &AsyncClient) {
    info!("Started publishing sensor data");

    loop {
        // randon temp
        let mut temp = rand::random_range(-10.0..150.0);

        // simulate 2% error rate
        if rand::random_range(1..100) <= 2 {
            temp = rand::random_range(-4000.0..4000.0);
        }

        let payload = TemperatureData {
            id: "temperature".to_string(),
            temperature: temp,
        };

        // serialize sensor data using postcard
        let bytes = postcard::to_allocvec(&payload).expect("[Sensor] sensor data must serialize");

        match client
            .publish("sensors/temperature", QoS::AtLeastOnce, false, bytes)
            .await
        {
            Ok(_) => {
                info!("Sensed {:.2}°C", temp);
            }
            Err(e) => {
                error!("Could not produce data: {}", e);
            }
        }

        sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let conf = &SensorConfig {
        broker_ip: get_env_or("BROKER_IP", "127.0.0.1".to_string()),
        broker_port: get_env_or("BROKER_PORT", 1883),
    };

    match init_event_loop(conf).await {
        Ok((client, mut evtloop)) => {
            // run mqtt loop in the background
            tokio::spawn(async move {
                loop {
                    match evtloop.poll().await {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Communication error: {e}");
                            error!("Retrying in 1 second");
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
            });

            info!("Ready to read from broker");
            publish_events(&client).await;
        }
        Err(e) => {
            error!("Initialization failed: {}", e);
        }
    };
}
