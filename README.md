# Log Aggregator with Alerting System

This project implements a log aggregator with alerting system as described in the guidelines.

## Part 1: Data Ingestion

The first part of the project sets up a mock application stack with Nginx and a simple Golang API using Chi framework.

### Components

- **Nginx**: Acts as a reverse proxy for the API
- **Golang API**: A simple API built with Chi framework that logs requests

### Directory Structure

```
.
├── logs/           # API logs directory
├── Dockerfile      # Dockerfile for the API
├── go.mod          # Go module file
└── main.go         # API source code
├── nginx/
│   └── conf.d/         # Nginx configuration
│       └── default.conf # Nginx configuration file
├── docker-compose.yml  # Docker Compose configuration
└── README.md           # This file
```

### How to Run

1. Make sure you have Docker and Docker Compose installed
2. Clone this repository
3. Run the following command:

```bash
docker-compose up -d
```

4. Access the API through Nginx at http://localhost
5. The API will respond with "Hello World from Chi!"
6. Check the health endpoint at http://localhost/health

### Logs

- Nginx logs are stored in `./nginx/logs/`
- APP logs are stored in `./logs/app.log`

## Next Steps

The next parts of the project will involve:
- Setting up Kafka for log collection
- Configuring FluentBit to ship logs to Kafka
- Implementing log parsing and enrichment
- Setting up storage and query layer
- Implementing real-time alerting