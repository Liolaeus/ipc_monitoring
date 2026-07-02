use std::time::Duration;

use common::{TemperatureData, get_env_or};
use rumqttc::{AsyncClient, ClientError, Event, EventLoop, MqttOptions, Packet, QoS};

struct ProcessorConfig {
    broker_ip: String,
    broker_port: u16,
}

async fn init_event_loop(conf: &ProcessorConfig) -> Result<(AsyncClient, EventLoop), ClientError> {
    let mut mqtt_options = MqttOptions::new(
        "monitoring-processor",
        conf.broker_ip.clone(),
        conf.broker_port,
    );
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    Ok(AsyncClient::new(mqtt_options, 10))
}

async fn handle_events(mut eventloop: EventLoop, client: AsyncClient) {
    let mut count = 0;
    let mut sum = 0.0;
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    loop {
        match eventloop.poll().await {
            // on connection error wait 1 sec and try again
            Err(e) => {
                eprintln!("[Processor] Communication error: {e}");
                eprintln!("[Processor] Retrying in 1 seconds");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            // on connection initialization, subscribe to temperature
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                println!("[Processor] Connected to broker");
                client
                    .subscribe("sensors/temperature", QoS::AtLeastOnce)
                    .await
                    .expect("[Processor] failed to subscribe");
            }

            // handle messages from sensors/temperature
            Ok(event) => {
                if let Event::Incoming(Packet::Publish(publish)) = event {
                    count += 1;

                    match postcard::from_bytes::<TemperatureData>(&publish.payload) {
                        Err(e) => eprintln!("[Processor] Error parsing message: {}", e),

                        Ok(data) => {
                            count += 1;
                            sum += data.temperature;

                            if data.temperature < min {
                                min = data.temperature;
                            }
                            if data.temperature > max {
                                max = data.temperature;
                            }

                            println!(
                                "[Processor] Count: {} | Avg: {:.2}°C | Min: {:.2}°C | Max: \
                                 {:.2}°C",
                                count,
                                (sum / count as f32),
                                min,
                                max
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
    let conf = &ProcessorConfig {
        broker_ip: get_env_or("BROKER_IP", "127.0.0.1".to_string()),
        broker_port: get_env_or("BROKER_PORT", 1883),
    };

    match init_event_loop(conf).await {
        Ok((client, evtloop)) => {
            println!("[Processor] Ready to read from broker.");
            handle_events(evtloop, client).await;
        }
        Err(e) => {
            eprintln!("[Processor] Initialization failed: {}", e);
        }
    };
}
