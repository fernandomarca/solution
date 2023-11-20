use chrono::Utc;
#[allow(clippy::useless_attribute)]
use rabbitmq_stream_client::error::ProducerPublishError;
use rabbitmq_stream_client::error::StreamCreateError;
use rabbitmq_stream_client::types::ByteCapacity;
use rabbitmq_stream_client::types::Delivery;
use rabbitmq_stream_client::types::Message;
use rabbitmq_stream_client::types::OffsetSpecification;
use rabbitmq_stream_client::ConsumerHandle;
use rabbitmq_stream_client::Environment;
use rabbitmq_stream_client::TlsConfiguration;
use rocket::futures::StreamExt;
use rocket::tokio::task;
use std::future::Future;

pub struct RabbitMqAdapter {
    pub environment: Environment,
}

#[allow(dead_code)]
pub enum Offset {
    First,
    Last,
    Next,
    Offset(u64),
}

impl RabbitMqAdapter {
    pub async fn new() -> Self {
        // let tls_configuration = Self::tls_config();
        let environment = Environment::builder()
            .host("localhost")
            .port(5552)
            .username("rabbitmq")
            .password("rabbitmq")
            .heartbeat(60)
            // .tls(tls_configuration)
            .build()
            .await
            .expect("Failed to create environment");

        Self { environment }
    }

    #[allow(dead_code)]
    pub async fn publish(
        &self,
        message: &[u8],
        stream_name: &str,
    ) -> Result<(), ProducerPublishError> {
        let producer_builder = self
            .environment
            .producer()
            .build(stream_name)
            .await
            .expect("Failed to create producer");

        let message_build = Message::builder()
            .message_annotations()
            .insert("test", 1)
            .message_builder()
            .application_properties()
            .insert("test", 2)
            .message_builder()
            .properties()
            .content_encoding("application/json")
            .absolute_expiry_time(Utc::now())
            .message_builder()
            .body(message)
            .build();

        let publish_result = producer_builder.send_with_confirm(message_build).await;

        match publish_result {
            Ok(status) => {
                println!(
                    "Message published {} {}",
                    status.confirmed(),
                    status.publishing_id()
                );
                producer_builder
                    .close()
                    .await
                    .expect("Failed to close producer");
                Ok(())
            }
            Err(e) => {
                println!("Error publishing message: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn consumer<F, Fut>(
        &self,
        stream_name: &str,
        offset: Option<Offset>,
        callback: F,
    ) -> ConsumerHandle
    where
        F: Fn(Delivery) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let offset = match offset {
            Some(Offset::First) => OffsetSpecification::First,
            Some(Offset::Last) => OffsetSpecification::Last,
            Some(Offset::Next) => OffsetSpecification::Next,
            Some(Offset::Offset(offset)) => OffsetSpecification::Offset(offset),
            None => OffsetSpecification::Next,
        };

        let mut consumer = self
            .environment
            .consumer()
            // .name("tag")
            .offset(offset)
            .build(stream_name)
            .await
            .expect("failed to create consumer");

        let handle = consumer.handle();
        task::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                match delivery {
                    Ok(delivery) => {
                        callback(delivery).await;
                    }
                    Err(e) => {
                        println!("Error receiving message: {:?}", e);
                    }
                }
            }
        });
        handle
    }

    pub async fn create_stream(
        &self,
        stream_name: &str,
        capacity_gb: u64,
    ) -> Result<(), StreamCreateError> {
        self.environment
            .stream_creator()
            .max_length(ByteCapacity::GB(capacity_gb))
            .create(stream_name)
            .await
    }

    #[allow(dead_code)]
    fn tls_config() -> TlsConfiguration {
        // TlsConfiguration::builder()
        //     .add_root_certificates(String::from(".ci/certs/ca_certificate.pem"))
        //     .build()

        // Use this configuration if you want to trust the certificates
        // without providing the root certificate and the client certificates

        // TlsConfiguration::builder()
        //     .add_root_certificates(String::from(".ci/certs/ca_certificate.pem"))
        //     .add_client_certificates_keys(
        //         String::from(".ci/certs/client_certificate.pem"),
        //         String::from("certs/client_key.pem"),
        //     )
        //     .build()

        // Use this configuration if you want to trust the certificates
        // without providing the root certificate
        TlsConfiguration::builder().trust_certificates(true).build()
    }
}
