use std::time::Duration;

use common::{TemperatureData, get_env_or};
use rumqttc::{AsyncClient, ClientError, Event, EventLoop, MqttOptions, Packet, QoS};

struct AlerterConfig {
    broker_ip: String,
    broker_port: u16,
    bound_low: f32,
    bound_high: f32,
}

async fn init_event_loop(conf: &AlerterConfig) -> Result<(AsyncClient, EventLoop), ClientError> {
    let mut mqtt_options = MqttOptions::new(
        "monitoring-alerter",
        conf.broker_ip.clone(),
        conf.broker_port,
    );
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    Ok(AsyncClient::new(mqtt_options, 10))
}

async fn handle_alerts(mut eventloop: EventLoop, client: AsyncClient, conf: &AlerterConfig) {
    let mut count = 0.0;
    let mut errors = 0.0;
    let temp_range = conf.bound_low..conf.bound_high;

    loop {
        match eventloop.poll().await {
            // on connection error wait 1 sec and try again
            Err(e) => {
                eprintln!("[Alerter] Communication error: {e}");
                eprintln!("[Alerter] Retrying in 1 seconds");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            // on connection initialization, subscribe to temperature
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                println!("[Alerter] Connected to broker");
                client
                    .subscribe("sensors/temperature", QoS::AtLeastOnce)
                    .await
                    .expect("[Alerter] failed to subscribe");
            }

            // handle messages from sensors/temperature
            Ok(event) => {
                if let Event::Incoming(Packet::Publish(publish)) = event {
                    count += 1.0;

                    match postcard::from_bytes::<TemperatureData>(&publish.payload) {
                        Err(e) => eprintln!("[Alerter] Error parsing message: {}", e),

                        Ok(data) => {
                            if temp_range.contains(&data.temperature) {
                                continue;
                            }

                            errors += 1.0;
                            println!(
                                "[Alerter] Abnormal temperature detected: {:.2}°C | error rate: \
                                 {:.2}%",
                                data.temperature,
                                (errors / count) * 100.0
                            );
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let conf = &AlerterConfig {
        broker_ip: get_env_or("BROKER_IP", "127.0.0.1".to_string()),
        broker_port: get_env_or("BROKER_PORT", 1883),
        bound_low: get_env_or("BOUND_LOW", -10.0),
        bound_high: get_env_or("BOUND_HIGH", 150.0),
    };

    match init_event_loop(conf).await {
        Ok((client, evtloop)) => {
            println!("[Alerter] Ready to read from broker.");
            handle_alerts(evtloop, client, conf).await;
        }
        Err(e) => {
            eprintln!("[Alerter] Initialization failed: {}", e);
            return;
        }
    };
}
