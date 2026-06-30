use common::{TemperatureData, get_env_or};
use rumqttc::{AsyncClient, ClientError, EventLoop, MqttOptions, QoS};
use std::time::Duration;
use tokio::time::sleep;

struct SensorConfig {
    broker_ip: String,
    broker_port: u16,
}

async fn init_event_loop(conf: &SensorConfig) -> Result<(EventLoop, AsyncClient), ClientError> {
    let mut mqtt_options = MqttOptions::new(
        "monitoring-sensor",
        conf.broker_ip.clone(),
        conf.broker_port,
    );
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqtt_options, 10);

    Ok((eventloop, client))
}

async fn publish_events(client: &AsyncClient) {
    println!("[Sensor] Started publishing sensor data");

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

        if let Ok(json) = serde_json::to_string(&payload) {
            match client
                .publish("sensors/temperature", QoS::AtLeastOnce, false, json)
                .await
            {
                Ok(_) => {
                    println!("[Sensor] Sensed {:.2}°C", temp)
                }
                Err(e) => {
                    eprintln!("[Sensor] Could not produce data: {}", e);
                }
            }
        }

        sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let conf = &SensorConfig {
        broker_ip: get_env_or("BROKER_IP", "127.0.0.1".to_string()),
        broker_port: get_env_or("BROKER_PORT", 1883),
    };

    match init_event_loop(conf).await {
        Ok((mut evtloop, client)) => {
            // run mqtt loop in the background
            tokio::spawn(async move {
                loop {
                    match evtloop.poll().await {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("[Sensor] Communication error: {e}");
                            eprintln!("[Sensor] Retrying in 1 seconds");
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
            });

            println!("[Sensor] Ready to read from broker.");
            publish_events(&client).await;
        }
        Err(e) => {
            eprintln!("[Sensor] Initialization failed: {}", e);
        }
    };
}
