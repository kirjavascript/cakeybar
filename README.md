<div align="center">
    <img src="misc/logo.svg.png" alt="cakeybar">
    <br>
</div>
<br>

a customizable statusbar for your windowmanager

* multibar/multimonitor support
* expressive theming with CSS
* inter-process communication
* hot config reloading
* floating windows
* windowmanager neutral config
* system tray integration
* more rice than feudal japan

cakeybar is written in Rust using GTK and XCB

*work in progress*  
currently testing in i3wm/bspwm

[request a feature or file a bug](https://github.com/kirjavascript/cakeybar/issues)

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

## CLI options

```
    -h, --help                 Prints help information
    -M, --monitors             Shows information about monitors
    -V, --version              Prints version information
    -w, --watch                Watch config files and reload on changes
    -c, --config <FILE>         Specify a config path
    -m, --message <MESSAGE>    Send an IPC message
```

## command syntax

used for IPC and in the **command-input** component

`show [selector-list]`  
`hide [selector-list]`

used to show/hide windows. example: `show .stats, #bar`

`reload config [path]`  
`reload theme [path]`

used to reload (or change) the theme or the entire config. the path is optional

`focus [selector]`

(currently) used to focus on a **command-input** component. example: `focus #autocomplete`

## configuration

[TOML](https://github.com/toml-lang/toml) is used in 'normal' config files and CSS is used for theming

[CSS overview](https://developer.gnome.org/gtk3/stable/chap-css-overview.html)  
[CSS properties](https://developer.gnome.org/gtk3/stable/chap-css-properties.html)

The [Pango Text Attribute Markup Language](https://developer.gnome.org/pango/stable/PangoMarkupFormat.html) adds hyperlinks and other formatting options for format strings

see the [examples](examples) for more

#### global properties

```toml
# path to theme. paths can be relative or absolute
theme = "theme.css"

# dictate IPC usage
enable-ipc = true
```

### statusbar config

```toml
# define a bar with the name `bar_name`
[bar.bar_name]

# provide a class for the bar
class = "class-name"

# monitor index the bar appears on. a list of monitors can be seen with `cakeybar -M`
monitor = 0

# where to show the bar. options are: top | bottom
position = "top"

# a list of components to add to the bar, identified by name
layout = [ "component", "names", "go", "here" ]

# if enabled, will bind workspace next/prev actions to scroll events
workspace-scroll = false

# decide if the bar should reserve space on the desktop
reserve-space = true

# disable shadows in compton
disable-shadow = true
```

you can define as many bars as you like as long as they have unique names. the name is also used as the CSS selector for that bar: `#bar_name`

bars and floating windows will add a `.focused` class when you mouseover them

### floating window config

```toml
# define a floating window with the name `float_name`
[float.float_name]

# provide a class for the window
class = "class-name"

# provide a title for the window
title = ""

# a list of components to add to the window, identified by name
layout = [ "component", "names", "go", "here" ]

...TBC...
```

### component config

components can be used in either bars or floating windows

#### common properties

```toml
# define a component with the name `component_name`
[component.component_name]

# the only required property for a component is **type**
# values presented in this documentation are defaults
type = "void"

# components can be styled with `#component_name` and `.class-name`
class = "class-name"

# alignments can be: start | end | center | fill
halign = "void"
valign = "void"

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
direction = "horizontal"
layout = [ "component", "names", "go", "here" ]
```

possible directions: `horizontal` or `vertical`

#### command-input

an input box with autosuggestions and history for running programs and commands

```toml
[component.autocomplete]
type = "command-input"
history = 1000
```

will run installed programs, or prefix with `:` to run an IPC-style command (eg `:show #info`)

`Tab` is used for completing a word and `Right` is used for completing to the end

see the command syntax section to see how to focus the input

#### window-title

displays the current active window's title

```toml
[component.window_title]
type = "window-title"
format = "{title}"
truncate = 100
```

#### workspaces

```toml
[component.workspace_list]
type = "workspaces"
show-all = false # show workspaces from every monitor
spacing = 0 # gap between items
format = "{number}" # symbols are; number, name
```

each `label` element in a workspace can have the focused, visible and urgent classes which can be targeted with `#workspace_list label .focused`

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

#### backlight

```toml
[component.backlight]
type = "backlight"
format = "{percent}"
```

#### disk

```toml
[component.disk]
type = "disk"
mounts = ["/"] # omit to show all
format = "{free}" # symbols are; free, used, total, fs, mount
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
icon-spacing = 0
```

the `background-color` style property needs to be set explicitly for it to work

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
