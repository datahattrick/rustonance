# Rustonance

## Usage

The required parameter `DISCORD_TOKEN` must be set for the bot to work. Provide your discord application token and invite the bot to your server.

### Helm
A helm chart is provided via this respository in the directory [helm/](helm).

```
helm show values rustonance/helm/ > values.yaml
# Edit values.yaml and populate `discordToken`
helm upgrade --namespace rustonance --create-namespace -i rustonance rustonance/helm/ --values values.yaml
```

### Docker

#### Manually via CLI
```
docker run -e DISCORD_TOKEN="your_token" ghcr.io/datahattrick/rustonance:latest
```

#### Docker Compose
An example [docker compose file](docker-compose.yaml) is provided.

```
# Edit docker-compose.yaml to include your DISCORD_TOKEN
docker compose up -d
```

## TODO

- [ ] Implement Spotify playing
- [x] Implement Youtube playing 
- [x] Create queue logic
- [x] command stop
- [x] command pause
- [x] command skip
- [x] command leave
- [ ] command list
- [x] command play
- [ ] command seek
- [x] command resume
- [ ] command now (nowPlaying)
- [x] command repeat
- [ ] implement caching of queue
- [ ] implement nice messaging 
- [ ] Deal with play race condition
- [ ] Implement discord application creation link
