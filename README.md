<p align="center">
<a href="https://github.com/ChloeWKY/CopyClip/releases">
    <img src="./app-icon-gray.png" width=90 height=90>
</a>

<h2 align="center">Copy Clip</h2>

<p align="center">
    Versatile clipboard history tool that enables seamless clipboard management across MacOS, Linux, and Windows platforms. 
</p>
</p>

<br>

## Features:

**Persistent Presence**: The application's icon is ever-present in your system tray for easy access to your clipboard history at any time.

**Tray Menu Access**: Right-click (Windows) or click (Mac/Linux) on the Clipboard Companion icon to unveil the menu tray, where you can swiftly select and paste any stored clip.

**Dynamic Windows**: One-click access from the menu tray to open dedicated "Preferences," "Search," or "Help" windows, with intuitive top bar navigation for in-app window switching.

**Controlled Monitoring**: Toggle clipboard tracking on demand with the "Pause Monitoring" feature, resuming whenever you choose to ensure your clipboard captures only what you need.

**Advanced Searchability**: Employ powerful search capabilities within the app to locate specific clips by content, timestamp, or frequency of use, with smart display of recent clips in absence of search queries.

**Favorites Curation**: Highlight and access your most valued clips quickly by marking them as favorites through the "Search" interface, ensuring they're always at your fingertips.

**Priority Pins**: Elevate essential clips for top-tier accessibility by pinning them, allowing for immediate retrieval at the top of the system tray list.

**Selective Deletion**: Manage your clipboard with ease by removing unwanted clips individually, or in bulk, via the clear "Delete" options associated with each clip or from the search table header.

**Duplicate Management**: Enable "Auto Delete Duplications" in the "Preferences" to keep your clipboard history streamlined and free of redundancies.

**Customizable Viewing**: Tailor the number of clips displayed per page in the tray and set a maximum clip length to ensure a clean, organized view that fits your workflow.

**Multilingual Interface**: Seamlessly switch between languages, including Chinese, English, and German, directly from the "Preferences" window to cater to a diverse user base and enhance usability across global audiences.

## Download

Download latest version of Copy Clip from [Github releases](https://github.com/ChloeWKY/CopyClip/releases) for free.

Navigate to the [releases](https://github.com/ChloeWKY/CopyClip/releases) page and locate the "Assets" section, where the latest pre-compiled executables for various operating systems are available for download. 

The app supports MacOS, Windows and Ubuntu.
For other Linux distributions, you can [build](#build) the app from source.


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