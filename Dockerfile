# Using rust as base image
FROM rust:1.67

RUN apt update -y && \
    apt upgrade -y && \
    apt install -y sudo git ssh lsof xclip htop vim neovim tree curl wget neofetch make build-essential iputils-ping nmap net-tools netcat && \
    rm -rf /var/lib/apt/lists/*

COPY . /app

# Setting default work directory
WORKDIR /app

# Build the Rust application
RUN cargo build --release

# Expose server port
EXPOSE 8080

CMD ["./target/release/annealing"]