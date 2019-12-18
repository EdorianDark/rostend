# Rostend

Currently the only possibility to run several services inside of one docker container is to write
fragile shell scripts to start them.
Rostend tries to change that.
The target is to be a
Systemd compatible init system for containers.

To see it in action, go to examples/ an run  ````make_example.sh```` and build a docker container.
The ```.services``` are parsed and the services started accordingly inside the container.

Note that Rostend is more a proof of concept than finished, it should not be used for anything 
important yet.

