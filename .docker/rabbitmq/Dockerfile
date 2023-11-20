FROM rabbitmq:3.12.9-management

RUN rabbitmq-plugins enable --offline rabbitmq_management rabbitmq_stream rabbitmq_stream_management rabbitmq_shovel rabbitmq_shovel_management

ENV RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS="-rabbitmq_stream advertised_host localhost" 

# as portas 5551 e 5552 são usadas para o stream
# ports:
#       - "15672:15672"
#       - "5672:5672"
#       - "5551:5551"
#       - "5552:5552"