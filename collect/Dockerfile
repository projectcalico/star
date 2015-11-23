FROM jimmycuadra/rust:1.1.0
MAINTAINER Mesosphere <support@mesosphere.io>

ADD . /star
WORKDIR /star
ADD star-collect /usr/bin/collect
ADD probes.json /star/probes.json
CMD ["collect", "--resources-file=/star/probes.json"]
