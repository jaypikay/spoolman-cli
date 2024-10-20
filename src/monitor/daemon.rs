use crate::config::read_config;
use crate::spoolman::spool;

use futures::executor;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::File, time::Duration};
use tokio::time;

use daemonize::Daemonize;

use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: u32,
    weight: f32,
}

async fn handle_message(message: Message) {
    match spool::set_spool_weight(&message.id, &message.weight).await {
        Ok(_) => eprintln!("Request posted successfully"),
        Err(e) => eprintln!("Failed posting request: {}", e),
    }
    time::sleep(Duration::from_millis(200)).await;
}

#[tokio::main]
pub async fn mqtt_event_loop() {
    let config = read_config().unwrap();

    let mut mqttoptions =
        MqttOptions::new(config.mqtt.clientid, config.mqtt.host, config.mqtt.port);
    mqttoptions.set_credentials(config.mqtt.username, config.mqtt.password);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let status = client.try_subscribe(config.mqtt.topic, QoS::AtMostOnce);
    match &status {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to subscribe topic! Error = {e:?}");
        }
    }

    loop {
        let enter = executor::enter().unwrap();
        let event = eventloop.poll().await;

        match &event {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                match serde_json::from_slice::<Message>(&p.payload) {
                    Ok(message) => {
                        eprintln!("Message = {message:?}");
                        handle_message(message).await;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON message: {}", e)
                    }
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error in MQTT event loop: {}", e)
            }
        }

        time::sleep(Duration::from_secs(1)).await;

        drop(enter);
    }
}

//
pub fn run(foreground: bool) {
    println!("Starting monitoring daemon...");

    if !foreground {
        let stderr = File::create("/tmp/spool-daemon.err").unwrap();
        let stdout = File::create("/tmp/spool-daemon.out").unwrap();

        let daemonize = Daemonize::new()
            .pid_file("/tmp/spool-daemon.pid")
            .stdout(stdout)
            .stderr(stderr);

        match daemonize.start() {
            Ok(_) => {
                println!("Spoolman monitoring daemon started.");
                mqtt_event_loop();
            }
            Err(e) => println!("Error starting daemon: {}", e),
        }
    } else {
        println!("Spoolman daemon forground monitoring started.");
        //executor::block_on(mqtt_event_loop());
        mqtt_event_loop();
    }
}
