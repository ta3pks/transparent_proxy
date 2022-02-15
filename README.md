# Transparent Proxy

If you have a password protected proxy endpoint chrome does not unfortunately support that so you can simply run transparent proxy in between

for usage please run transparent_proxy --help

## Service workflow in macos

### Copy and load

```sh
sudo cp com.nefthias.transparent_proxy.plist /Library/LaunchDaemons &&
sudo launchctl load /Library/LaunchDaemons/com.nefthias.transparent_proxy.plist
```

### Unload and remove

```sh
sudo launchctl unload /Library/LaunchDaemons/com.nefthias.transparent_proxy.plist &&
sudo rm /Library/LaunchDaemons/com.nefthias.transparent_proxy.plist
```

### Start

```sh
sudo launchctl start com.nefthias.transparent_proxy
```

### Stop

```sh
sudo launchctl stop com.nefthias.transparent_proxy
```

### Restart

```sh
sudo launchctl restart com.nefthias.transparent_proxy
```

### Trace logs

```sh
sudo tail -f /var/log/com.nefthias.transparent_proxy.log
```
