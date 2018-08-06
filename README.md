<div align="center">
    <img src="misc/logo.svg.png" alt="cakeybar">
    <br>
</div>
<br>

**cakeybar** is a user friendly tool for creating custom statusbars

* multibar/multimonitor support
* expressive theming with CSS
* windowmanager neutral design
* system tray integration
* more rice than feudal japan

*work in progress*  
configuration is not stable yet  
currently testing in i3wm/bspwm

## quickstart

### from source

```bash
# install rustup (if you don't have cargo)
curl https://sh.rustup.rs -sSf | sh

# clone repo
git clone https://github.com/kirjavascript/cakeybar.git
cd cakeybar

# run example
cargo run --release -- -c examples/darkblue/config.toml
```

## configuration

[TOML](https://github.com/toml-lang/toml) is used in 'normal' config files and CSS is used for theming

[CSS overview](https://developer.gnome.org/gtk3/stable/chap-css-overview.html)  
[CSS properties](https://developer.gnome.org/gtk3/stable/chap-css-properties.html)

see the [examples](examples) for more

#### global properties

```toml
theme = "theme.css" # paths can be relative or absolute
```

### bar config

```toml
[bar.bar_name]
monitor = 0
position = "top" # "top" | "bottom"
layout = [ "component", "names", "go", "here" ]
scroll-workspace = false
```

you can define as many bars as you like as long as they have unique ids. the id is also used as the CSS selector for that bar: `#bar_name`

### component config

#### common properties

```toml
# define a component with the name `component_name`
[component.component_name]

# the only required property for a component is **type**
type = "image"

# components can be styled with `#component_name` and `.class-name`
class = "class-name"

# alignments can be: `start | end | center | fill`
halign = "center"
valign = "fill"

# the fixed property changes the component position from relative to absolute
# disabling pass-through allows the fixed component to capture mouse events
fixed = false
pass-through = true

# the update interval in seconds
interval = 3

# format strings use a basic syntax for replacing named symbols with data
format = "label: {symbol-name}"

# to print a literal curly bracket, duplicate the character `{{` or `}}`
```

#### image

```toml
[component.start_icon]
type = "image"
src = "window.png"
```

#### clock

```toml
[component.time]
type = "clock"
format = "%Y-%m-%d %H:%M:%S"
```

formatting differs from other components: [formatting guide](https://docs.rs/chrono/0.4.2/chrono/format/strftime/index.html)

#### container

```toml
[component.stats_box]
type = "container"
spacing = 0
direction = "vertical"
layout = [ "component", "names", "go", "here" ]
```

can be used to create more complex layouts or group components to share between bars

possible directions: `column | row` or `horizontal | vertical`


#### workspaces

```toml
[component.workspace_list]
type = "workspaces"
show-all = false # show workspaces from every window
show-name = false # show full name or just index
```

each `label` element in a workspace can have the focused, visibile and urgent classes which can be targeted like `#workspace_list label .focused`

#### mode

```toml
[component.current_mode]
type = "mode"
```

will be hidden in the default mode

#### window

```toml
[component.window_title]
type = "window"
```

#### bandwidth

```toml
[component.download]
type = "bandwidth"
interfaces = ["eth0"] # omit to show all
format = "{down/s}" # symbols are; name, down/s, up/s, down/total, up/total
```

#### ip

```toml
[component.ip_address]
type = "ip"
interfaces = ["eth0"] # omit to show all
format = "{ipv4}" # symbols are; name, ipv4, ipv6
```

#### battery

```toml
[component.battery]
type = "battery"
battery = "BAT0"
adapter = "AC"
```

you can target the class `#battery.plugged` when AC is plugged in

classes for battery charge are: `full | high | medium | low`

use `ls /sys/class/power_supply/` to see devices

#### disk

```toml
[component.disk]
type = "disk"
mounts = ["/"] # omit to show all
format = "{free}" # symbols are; free, total, type, name, path
```

#### tray

```toml
[component.tray]
type = "tray"
icon-size = 20
```

the `background-color` style needs to be set explicitly for it to work

#### script

```toml
[component.load_averages]
type = "script"
src = "uptime | sed -r \"s/.*average: (.*)$/\\1/\""
```

#### equalizer

```toml
[component.eq]
type = "equalizer"
```

experimental pulseaudio visualizer

#### dropdown

```toml
[component.dropdown]
type = "dropdown"
items = [
    { label = "web browser", command = "firefox" },
    { label = "screenshot", command = "xfce4-screenshooter" },
    { label = "background", children = [
        { label = "forest", command = "feh --bg-fill forest.png" },
        { label = "mountain", command = "feh --bg-fill mountain.png" },
    ] },
]
```

a gtk-context style dropdown menu
