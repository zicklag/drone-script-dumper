# Drone Script Dumper

This is just a super tiny script that will dump all of the data from the `commands` portions of a `.drone.yml` file so that you can paste them into a terminal without having to remove all of the YAML formatting manually. The script has no configuration and will just read the .drone.yml file in the current directory.

## Usage

In a directory with a `.drone.yml` file ( in this case the [Drone](https://github.com/drone/drone) repo ):

```
$ drone-script-dumper
Pipeline: linux-amd64
============
Step: test
--------
go test -v ./...

Step: build
--------
go build -ldflags "-extldflags \\"-static\\"" -o release/linux/amd64/drone-server github.com/drone/drone/cmd/drone-server
CGO_ENABLED=0 go build -o release/linux/amd64/drone-agent github.com/drone/drone/cmd/drone-agent
CGO_ENABLED=0 go build -o release/linux/amd64/drone-controller github.com/drone/drone/cmd/drone-controller

Step: publish_agent
--------

Step: publish_controller
--------

Step: publish_server
--------

Pipeline: linux-arm
============
Step: test
--------
go test -v ./...

Step: build
--------
go build -ldflags "-extldflags \\"-static\\"" -o release/linux/arm/drone-server github.com/drone/drone/cmd/drone-server
CGO_ENABLED=0 go build -o release/linux/arm/drone-agent github.com/drone/drone/cmd/drone-agent
CGO_ENABLED=0 go build -o release/linux/arm/drone-controller github.com/drone/drone/cmd/drone-controller

Step: publish_agent
--------

Step: publish_controller
--------

Step: publish_server
--------

Pipeline: linux-arm64
============
Step: test
--------
go test -v ./...

Step: build
--------
go build -ldflags "-extldflags \\"-static\\"" -o release/linux/arm64/drone-server github.com/drone/drone/cmd/drone-server
CGO_ENABLED=0 go build -o release/linux/arm64/drone-agent github.com/drone/drone/cmd/drone-agent
CGO_ENABLED=0 go build -o release/linux/arm64/drone-controller github.com/drone/drone/cmd/drone-controller

Step: publish_agent
--------

Step: publish_controller
--------

Step: publish_server
--------

Pipeline: manifest
============
Step: server
--------

Step: controller
--------

Step: agent
--------
```
