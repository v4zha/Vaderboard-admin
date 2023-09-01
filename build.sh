#!/bin/bash

IMAGE_NAME="vaderboard-admin"
TAG="latest"

docker build -t $IMAGE_NAME:$TAG .

if [ $? -eq 0 ]; then
  docker run -p 8080:8080 --env-file .env $IMAGE_NAME:$TAG
  if [ $? -eq 0 ]; then
    echo "Docker container is running. Access the application at http://localhost:8080"
  else
    echo "Failed to run Docker container."
  fi
else
  echo "Failed to build Docker image."
fi
