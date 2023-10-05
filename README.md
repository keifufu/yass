# yet another sharex server (yass)

A self-hosted web-server to easily share files with ShareX.  
There are many others, but this one is mine :3

## Features

- Upload screenshots and files from ShareX
- Preview page for Images, Videos, Audio and other Files
- Code highlights for text-based files
- Short URLs to share
- Very lightweight
- Cross-platform
- Standalone executable

## Info

- Files are stored in separate directories based on the file type.  
  Eg: Images, Videos, Audio, Text, Files
- Filenames are: `filename-<id>.ext` where `<id>` is the key used in the URL

## Disclaimer

- It has no web-ui for managing uploaded files as I intend to use this with FileRun.
- This is only intended to be used by one person.
- A reverse proxy should be used to configure ssl and handle path prefixes if needed.
- All uploaded files are public by default (although that should be obvious).

## Getting Started

### With Docker

Using docker compose:

```yaml
version: "3"
services:
  yass:
    image: keifufu/yass:latest
    container_name: yass
    restart: unless-stopped
    volumes:
      - ./config:/app/config
      - /data/ShareX:/data/ShareX
    ports:
      - 8080:8080
```

or docker run:

```
docker run -v ./config:/app/config keifufu/cf-ddns:latest
```

## Example configs:

- [yass.sxcu](https://raw.githubusercontent.com/keifufu/yass/main/yass.sxcu)
- [yass.sh](https://raw.githubusercontent.com/keifufu/yass/main/yass.sh)
- [nginx](https://github.com/keifufu/yass/blob/main/yass.nginx)
- [systemd service](https://github.com/keifufu/yass/blob/main/yass.service)

## TODO

- Improve preview page design
- OpenGraph embeds
