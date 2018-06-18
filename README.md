<div align="center">
    <img src="misc/logo.svg.png" alt="cakeybar">
    <br>
</div>
<br>

cakeybar is a customizable statusbar for your windowmanager

* multibar/multimonitor support
* easy layout model
* flexible theming with CSS
* components for i3
* system tray integration
* image support

**work in progress** currently only tested in i3

## build/run

```rust
cargo build --release
```

```rust
cargo run --release -- -h
```

## configuration

[toml](https://github.com/toml-lang/toml) is used in 'normal' config files and CSS is used for theming

[CSS overview](https://developer.gnome.org/gtk3/stable/chap-css-overview.html) [CSS properties](https://developer.gnome.org/gtk3/stable/chap-css-properties.html)

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
scroll_workspace = false
```

you can define as many bars as you like as long as they have unique ids. the id is also used as the CSS selector for that bar: `#bar_name`

### component config

#### common properties

```toml
[component.component_name]
type = "image"
class = "class-name"
halign = "center"
valign = "fill"
interval = 5
```

the only required property for a component is **type**

components can be styled with `#component_name` and `.class-name`

alignments can be: `start | end | center | fill`

interval is the update interval (for components that have one) in seconds

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

[formatting guide](https://docs.rs/chrono/0.4.2/chrono/format/strftime/index.html)

#### container

```toml
[component.stats_box]
type = "container"
spacing = 5 # optional
direction = "vertical"
layout = [ "component", "names", "go", "here" ]
```

can be used to create more complex layouts or group components to share between bars

direction can be: `column | row` or `horizontal | vertical`


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

#### i3workspace

```toml
[component.workspaces]
type = "i3workspace"
show_all = false # show workspaces from every window
show_name = false # show full name or just index
```

each `label` element in a workspace can have the focused, visibile and urgent classes which can be targeted like `#workspace_name label .focused`

#### i3mode

```toml
[component.mode]
type = "i3mode"
```

will be hidden in the default mode

#### i3window

```toml
[component.window]
type = "i3window"
```

#### bandwidth

```toml
[component.download]
type = "bandwidth"
interface = "eth0" # if omitted, uses the first device
```

#### ip

```toml
[component.ip_address]
type = "ip"
interface = "eth0" # if omitted, uses the first device
# ipv6 = true
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

#### tray

```toml
[component.tray]
type = "tray"
icon_size = 20
```

only one system tray can be active at a time

`background-color` needs to be set explicitly but otherwise styles work as normal
