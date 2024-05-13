<p align="center">
<a href="https://github.com/ChloeWKY/CopyClip/releases">
    <img src="./app-icon.png" width=90 height=90>
</a>

<h2 align="center">Copy Clip</h2>

<p align="center">
    Versatile clipboard history tool that enables seamless clipboard management across MacOS, Linux, and Windows platforms. 
</p>
</p>

<br>


## Disclaimer

### Privacy

This app stores your clipboard history locally
and will not send any data to servers.

This is an open-source project,
and you can review the source code to ensure no malpractice.

Alternatively, if you are not familiar with Rust,
you can use [Wireshark](https://www.wireshark.org/)
or any other packet capture tools to monitor the app's network traffic.

On MacOS, you can use the system `Activity Monitor`
to monitor the app's network traffic.
![Activity Monitor](./docs/imgs/macos_system_activity_monitor.png)

On Windows, you can use the system `Task Manager`
to monitor the app's network traffic.
![Task Manager](./docs/imgs/windows_system_task_manager.png)

## Feedbacks

If you have any feedback,
including feature requests, bug reports, etc.,
please open an [issue](https://github.com/ChloeWKY/CopyClip/issues).

We will do our best to address the issue.

## Configuration

Below is an explanation of the app's configurations.

| Name                        | Default Value | Description                                                                                      |
|-----------------------------|---------------|--------------------------------------------------------------------------------------------------|
| clips per page              | 20            | Defines the number of clips to display on one page in the tray, with a maximum value of 50. Displaying too many can cause the system tray menu to overflow. |
| max clip length             | 50            | When a clip is too long, it's truncated to stay under max-clip-length to fit in the tray.       |
| log level filter            | info          | Sets the app's log level: `trace`, `info`, `debug`, `warn`, `error`, `off`—from most to least detailed. |
| dark mode                   | off           | Toggles dark mode on or off.                                                                     |
| Enable Auto Delete Duplications | false    | If set to true, the app will delete a previous instance of a clip if it is copied again.        |
| Pinned Clips Add            | None          | Text to be pinned.                                                                               |
| Pinned Clips Remove         | None          | Text of the pinned clip you want to remove.                                                      |

More configuration options are on the way.

## Install

The app supports MacOS, Windows and Ubuntu.
For other Linux distributions, you can [build](#build) the app from source.

Download link: Navigate to the [releases](https://github.com/ChloeWKY/CopyClip/releases) page and locate the "Assets" section, where the latest pre-compiled executables for various operating systems are available for download. Please select the file appropriate for your system to proceed.

### MacOS

Since `sqlite3` is pre-installed on the system,
no additional dependencies are needed.

Simply copy the app to the Applications folder.

### Linux

The `xcb` dependency is required to monitor the clipboard.

Use `dpkg` to install the `deb` bundle.

## Known Issues

### MacOS Security Policy

The Mac aarch64 build may encounter issues with macOS security policies;
Apple requires developers to join a $99/year program
to be recognized as trusted.

You may need to manually run the following commands:

```bash
sudo spctl --master-disable
sudo xattr -r -d com.apple.quarantine {{the location of your app}}
```

If the problem has not been solved, use the x64 build.

### Other Issues

If you encounter additional issues, open an issue with the log file attached, and include details about your system.

Please set the log mode to trace to record the most detailed log possible.

On MacOS, the log file is located at ~/Library/Logs/org.eu.huazifan.copyclip/log.

On Windows, the log file is located at C:\Users\<username>\AppData\Roaming\org.eu.huazifan.copyclip\logs\log.

## Build

### Download the source code

```bash
git clone https://github.com/ChloeWKY/CopyClip.git
cd CopyClip
```

### Prerequisites

Ensure the following tools are installed:

- Rust
- Node.js
- npm
- wasm32-unknown-unknown
- tauri-cli
- trunk
- wasm-opt
- tailwindcss

To install `Rust`, refer to [rustup](https://www.rust-lang.org/tools/install).

To install `Node.js` and `npm`,
visit [Get Node.js](https://nodejs.org/en/download).

Install other dependencies with the following command:

```bash
# wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

# Install Dependencies
cargo install tauri-cli trunk wasm-opt

# Install npm
npm install
```

For MacOS, install `XCode` and `XCode Command Line Tools`:

```bash
# Installing XCode Command Line Tools
# XCode needs to be installed from App Store
xcode-select --install
```

For linux, you need to install additional dependencies: - `libxcb*`

```bash
sudo apt-get update
sudo apt install libdbus-1-dev libwebkit2gtk-4.0-dev build-essential \
    curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev \
    librsvg2-dev xcb libxcb-randr0-dev libxcb-xtest0-dev libxcb-xinerama0-dev \
    libxcb-shape0-dev libxcb-xkb-dev libxcb-xfixes0-dev
```

For NixOS, use the following command in the current directory to install all dependencies:

```bash
nix develop
```

### Build The App

```bash
cargo tauri build
```

During the build process, you may encounter known issues on MacOS:
- If `mac-notification-sys v0.6.1` fails to build, this is a known issue which is sometimes caused by using `nix`. We are currently seeking a solution. For updates, please refer to [this issue](https://github.com/Alex222222222222/CopyClip/issues/88).
- If you run into problems with `cc` or `ld`, try the following:
  - Delete the `./target` directory(`rm -rf ./target`) and rebuild.
  - Remove `/Library/Developer/CommandLineTools` using `sudo rm -rf /Library/Developer/CommandLineTools`, reinstall `XCode Command Line Tools`, and then rebuild.

### Run

To automatically rebuild `TailwindCSS` when the source code changes,
run the following command in a separate terminal:

```bash
npx tailwindcss -w
```

Then, start the app with:

```bash
cargo tauri dev
```

## Contributing

See [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) for more information.

## TODO

- [x] Search Page
- [x] Configuration Page
- [ ] Explanation for configurations
- [ ] Sign the app for MacOS builds
- [ ] Change the app icon so it is visible against white backgrounds
- [x] Export the history to a file
  - [ ] History import functionality

## Contributors

<!-- readme: contributors -start -->
<table>
<tr>
    <td align="center">
        <a href="https://github.com/Alex222222222222">
            <img src="https://avatars.githubusercontent.com/u/61620490?v=4" width="100;" alt="Alex222222222222"/>
            <br />
            <sub><b>Zifan Hua</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/ChloeWKY">
            <img src="https://avatars.githubusercontent.com/u/118044032?v=4" width="100;" alt="ChloeWKY"/>
            <br />
            <sub><b>Chloe</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/LingkKang">
            <img src="https://avatars.githubusercontent.com/u/104191582?v=4" width="100;" alt="LingkKang"/>
            <br />
            <sub><b>Lingkang</b></sub>
        </a>
    </td></tr>
</table>
<!-- readme: contributors -end -->