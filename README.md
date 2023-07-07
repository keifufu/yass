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

## Usage

- Download the zip from [Releases](https://github.com/keifufu/yass/releases)
- Configure config.toml
- Run yass on system startup
- Import [yass.sxcu](https://raw.githubusercontent.com/keifufu/yass/main/yass.sxcu) and set it as your default uploader for Images, Text and Files  
  (You will need to modify the domain and api-key)

## Example Nginx config:

```nginxconf
server {
  listen 443 ssl http2;
  listen [::]:443 ssl http2;
  server_name your.domain.com;

  # This makes sure to cancel the request immediately if the api key is invalid.
  # With cloudflare however, cf buffers the request so it might seem like it's
  # susceptible by a DoS attack, however it's really not. Once cloudflare sends
  # the buffered request we immediately abort it if the api key is invalid.
  location / {
    client_max_body_size 0;
    client_body_buffer_size 1m;
    proxy_request_buffering off;
    proxy_pass http://127.0.0.1:8080;
    proxy_intercept_errors on;
    error_page 401 @error;
  }

  location @error {
    return 444;
  }
}
```

## TODO

- Improve preview page design
- OpenGraph embeds
