FROM jimmycuadra/rust:1.1.0
MAINTAINER Mesosphere <support@mesosphere.io>

ADD . /star
WORKDIR /star
ADD star-probe /usr/bin/probe
ADD probe.py /usr/bin/probe.py
RUN apt-get update
RUN apt-get install -y python
RUN apt-get install -y python-pip
RUN pip install requests
CMD ["probe", "--urls=http://frontend.default.cluster.local:80/status,http://backend.default.cluster.local:6379/status,http://172.18.18.101:9000/status"]
