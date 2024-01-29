# Start from the latest version of the Alpine Linux image
FROM alpine:latest

# Install the bash shell
RUN apk add --no-cache bash

# Add the files for our tool
ADD hello.sh /usr/local/bin/hello.sh

# Make the tool executable
RUN chmod +x /usr/local/bin/hello.sh

# Set the default command to run when the container starts
CMD ["/usr/local/bin/hello.sh"]
