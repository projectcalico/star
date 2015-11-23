FROM jimmycuadra/rust:1.1.0
MAINTAINER Mesosphere <support@mesosphere.io>

ADD . /star
WORKDIR /star
ADD star-probe /usr/bin/probe
CMD ["probe", "--urls=http://frontend.webapp.cluster.local:9000/status,http://database.webapp.cluster.local:9000/status,http://client1.client.cluster.local:9000/status,http://client2.client.cluster.local:9000/status"]
