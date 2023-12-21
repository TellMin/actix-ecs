# https://hub.docker.com/_/microsoft-dotnet
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /source

# copy everything else and build app
COPY aspnetwasm/. ./aspnetwasm/
WORKDIR /source/aspnetwasm
RUN dotnet publish -c release -o /app

# final stage/image
FROM rust:1.71 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
COPY --from=build /app/wwwroot ./wwwroot
EXPOSE 8080
ENTRYPOINT ["./target/release/actix-ecs"]
