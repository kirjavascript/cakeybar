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
# path to theme. paths can be relative or absolute
theme = "theme.css"
```

### bar config

```toml
# define a bar with the name `bar_name`
[bar.bar_name]

# which monitor the bar should show on
# a list of monitors can be seen with `cakeybar -M`
monitor = 0

# where to show the bar. options are: `top | bottom`
position = "top"

# a list of components to add to the bar, identified by name
layout = [ "component", "names", "go", "here" ]

# if enabled, will bind workspace next/prev actions to scroll events
workspace-scroll = false
```

you can define as many bars as you like as long as they have unique names. the name is also used as the CSS selector for that bar: `#bar_name`

### component config

#### common properties

```toml
# define a component with the name `component_name`
[component.component_name]

# the only required property for a component is **type**
# values presented in this documentation are defaults
type = "void"

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

an image

```toml
[component.start_icon]
type = "image"
src = "window.png"
```

#### container

a container to create more complex layouts and group components

```toml
[component.stats_box]
type = "container"
spacing = 0
direction = "vertical"
layout = [ "component", "names", "go", "here" ]
```

possible directions: `column | row` or `horizontal | vertical`

#### window

displays the current active window's title

```toml
[component.window_title]
type = "window"
format = "{title}"
```

#### workspaces

```toml
[component.workspace_list]
type = "workspaces"
show-all = false # show workspaces from every monitor
format = "{number}" # symbols are; number, name
```

each `label` element in a workspace can have the focused, visibile and urgent classes which can be targeted with `#workspace_list label .focused`

#### mode

```toml
[component.current_mode]
type = "mode"
format = "{mode}"
```

will be hidden in the default mode

#### cpu
```toml
[component.cpu]
type = "cpu"
format = "{usage}" # symbols are; usage, temp, dumbtemp
```

#### memory
```toml
[component.memory]
type = "memory"
format = "{free-pct}" # symbols are; total, free, free-pct, used, used-pct, swap-total, swap-used
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
format = "{percent}" # symbols are; percent, remaining, plugged
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

#### clock

```toml
[component.time]
type = "clock"
timestamp = "%Y-%m-%d %H:%M:%S"
format = "{timestamp}"
```

[timestamp formatting guide](https://docs.rs/chrono/0.4.2/chrono/format/strftime/index.html)

#### script

```toml
[component.load_averages]
type = "script"
src = '''
    uptime | sed -r "s/.*average: (.*)$/\\1/"
'''
format = "{stdout}" # symbols are; stdout, stderr, code
```

#### tray

```toml
[component.tray]
type = "tray"
icon-size = 20
```

the `background-color` style needs to be set explicitly for it to work

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
