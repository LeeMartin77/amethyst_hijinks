# Amethyst Hijinks

I'm interested in learning about Rust, and Amethyst looks like a really cool framework to play with making games. 

## Running in debian

There is a bit of a breaking bug against wayland in debian right now. In order to use the latest stable of amethyst, you'll need to do the following:

### Disable Wayland and use X11

If you don't do this, the system will crash immediately. 

simply un-comment the below line in ```/etc/gdm3/daemon.conf```

```#WaylandEnable=false```

### Set the project's rust compiler to 1.47.0

Unfortunately, this unviels another bug, where mouse events are busted. You then need to change the project's compiler with the below command:

```rustup override set 1.47.0```