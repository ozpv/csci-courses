# use ubuntu if you want
FROM debian:latest

# dir for container
WORKDIR /output

# install dev tools
RUN apt-get update && apt install -y g++

# copy into WORKDIR
# Don't need to copy when not running tests
# the server will mount /tmp/csci-courses/<uuid>
# for access to code
# COPY main.cpp .
COPY build.sh .

# make build script execuatable
# users, groups, other...
RUN chmod +x ./build.sh

# compile and run the binary
CMD ["./build.sh"]
