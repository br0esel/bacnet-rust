FROM dokken/ubuntu-20.04

RUN mkdir /bacnet-stack
WORKDIR /bacnet-stack
COPY ["./bacnet-stack", "."]

RUN apt-get install build-essential -y

WORKDIR /bacnet-stack
RUN make clean all

WORKDIR /bacnet-stack/bin
CMD [ "./bacserv", "1234" ]
# CMD [ "./bacwi", "-1" ]