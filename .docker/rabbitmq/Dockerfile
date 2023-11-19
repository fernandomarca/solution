FROM rabbitmq:3.12.9-management

RUN rabbitmq-plugins enable --offline rabbitmq_management rabbitmq_stream rabbitmq_stream_management rabbitmq_shovel rabbitmq_shovel_management

ENV RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS="-rabbitmq_stream advertised_host localhost" 