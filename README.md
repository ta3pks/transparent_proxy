# Transparent Proxy

If you have a password protected proxy endpoint chrome does not unfortunately support that so you can simply run transparent proxy in between

for usage please run transparent_proxy --help

## Service workflow in macos

### Copy and load

```sh
cp com.nefthias.transparent_proxy.plist ~/Library/LaunchAgents &&
launchctl load ~/Library/LaunchAgents/com.nefthias.transparent_proxy.plist
```

### Unload and remove

```sh
launchctl unload ~/Library/LaunchAgents/com.nefthias.transparent_proxy.plist &&
rm ~/Library/LaunchAgents/com.nefthias.transparent_proxy.plist
```

### Start

```sh
launchctl start com.nefthias.transparent_proxy
```

### Stop

```sh
launchctl stop com.nefthias.transparent_proxy
```

### Restart

```sh
launchctl restart com.nefthias.transparent_proxy
```

### Trace logs

```sh
tail -f ~/Library/Logs/com.nefthias.transparent_proxy.log
```
