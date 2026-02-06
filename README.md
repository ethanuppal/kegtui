# kegtui

> [!WARNING]
> kegtui does **NOT** create wrappers or claim legal rights to them. kegtui is **NOT** a substitute for using [Kegworks][kegworks] to create wrappers.

Terminal interface for [Kegworks][kegworks].

## Download

[![macOS Download](https://img.shields.io/badge/macOS-Download-green?logo=apple&logoColor=white)](https://github.com/ethanuppal/kegtui/releases)

### First-time setup

After clicking on this button and downloading, you'll have to open it by right-clicking on it from Finder:

![Right click on the app and then click Open](./images/how-to-open-first-time.png)

Subsequent opens won't require this step.

If you drag the app into your Applications folder, you can open it via Spotlight.

> [!TIP]
> If you get an alert saying that the `.app` is damaged, run the following command:
> ```bash
> xattr -d com.apple.quarantine <path/to/kegtui.app>
> ```

### Setup

I aim to automate steps 2 and 3, but for now they have to be done manually.

1. Follow the instructions in the setup wizard. If you already have Kegworks Winery installed, you can proceed.
2. Open Kegworks Winery and create a new wrapper using the latest Engine and Wrapper. The app will take a long time and hang when you create the wrapper. Fortunately, you will not need to use that app much more.
3. Open kegtui, navigate to the wrapper you just created, and `Launch` it. If all goes well, you should see the Kegworks GUI config menu open. Close this window once it has opened. Back in kegtui, run `Kill Processes`.
4. Install Steam via winetricks. You can do this via kegtui by choosing `Winetricks` and then uncommenting the line starting with `# steam.app = `. The winetricks file will be opened by default in your `$EDITOR`; if you didn't define this, it'll open in plain Vim. (If you are unable to install Steam via kegtui, you will unfortunately have to use the GUI app again. `Launch` the wrapper and use the Winetricks button to install Steam.)
5. Once Steam is installed, choose `Edit Config` and set the program path to `"/Program Files (x86)/Steam/steam.exe"`.
6. Finally, you should be able to `Launch` the wrapper and use Steam as normal.

Remember to exit Steam by selecting `Kill Processes` in kegtui when you're done --- this is the equivalent of doing `Cmd-Shift-K` in Whisky.

## Install (for development)
```
git clone https://github.com/ethanuppal/kegtui
cd kegtui
cargo install --path .
```

## Usage

Press `?` anywhere to view keybinds.

## Support [GCenx](https://github.com/Gcenx).

Wine on macOS is effecivtively made possible by him.
Please support him in any way possible!

- https://paypal.me/gcenx
- https://ko-fi.com/gcenx

[kegworks]: https://github.com/Kegworks-App/Kegworks
