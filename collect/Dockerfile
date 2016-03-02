FROM jimmycuadra/rust:1.1.0
MAINTAINER Mesosphere <support@mesosphere.io>

ADD . /star
WORKDIR /star
ADD star-collect /usr/bin/collect
ADD probes.json /star/probes.json
ADD probe.py /usr/bin/probe.py
RUN apt-get update
RUN apt-get install -y python
RUN apt-get install -y python-pip
RUN pip install requests

CMD ["collect", "--resources-file=/star/probes.json"]
