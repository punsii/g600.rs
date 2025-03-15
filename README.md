# Logitech G600 Mapping on Linux with Wayland and X11
Utility program for binding actions to keys on the Logitech G600 gaming mouse. Uses `ydotool` to support key combinations on x11 and Wayland. **Steps on how to make this program run automatically on startup are also written below.**

Supports 16 keys and the G-shift button for a total of 32 fast shortcuts.

Before running this program open the Logitech Gaming Software on a Windows or Mac OS machine. Assign the three basic mouse buttons to their standard functions. The G-shift button should be assigned to the G-shift function. All the remaining buttons (scroll left, scroll right, G7, ... G20) should be set to emulate (unique) keyboard keys (but not modifier keys).

## Usage

### Download ydotool
It is the newer xdotool. Link to installation is [here](https://gabrielstaples.com/ydotool-tutorial/#gsc.tab=0).
Add the ydotool daemon path to your bashrc for QOL, and to make this script work.

`export YDOTOOL_SOCKET="$HOME/.ydotool_socket"`

### Setup to make ydotool run on startup
At the time of writing, ydotool daemon needs to be startup manually. These steps make ydotool startup automatically when your linux OS starts up. We're using systemd.

1. **Create a systemd service file:**
Open a terminal and create a new service file for your executable. You can use nano or any text editor:

```bash
sudo nano /etc/systemd/system/ydotool.service
```

2. **Add the service configuration:**
Add the following content to the file:

```ini
[Unit]
Description=ydotoold service

[Service]
Type=simple
Restart=always
RestartSec=3
ExecStartPre=/bin/sleep 2
ExecStart=/usr/local/bin/ydotoold --socket-path="/your/socket/path/.ydotool_socket" --socket-own="<your_uid>:<your gid>"
ExecReload=/usr/bin/kill -HUP $MAINPID
KillMode=process
TimeoutSec=180

[Install]
WantedBy=basic.target

```

I'm not too familiar with packaging, so if you are having issues, it could be because your ydotoold was installed somewhere other than `/usr/local/bin/ydotoold`, your socketpath is mismatched with the steps later on in this readme, or your uid:gid is incorrect. You can find your uid and gid with `id -u` and `id -g`.

3. **Reload the systemd daemon:**
After creating the service file, reload the systemd daemon to recognize the new service:

```sh
sudo systemctl daemon-reload
```

4. **Enable the service:**
Enable the service so that it starts automatically on boot:

```sh
sudo systemctl enable ydotool.service
```

5. **Start the service:**
Start the service immediately:

```sh
sudo systemctl start ydotool.service
```

6. **Check the service status:**
Verify that the service is running correctly:

```sh
    sudo systemctl status ydotool.service
```

This will ensure that your ydotoold executable runs automatically when your system starts and will be restarted automatically if it fails.

### Setup Mapping Script
1. Clone this repository.
2. Open `g600.c` and fill in the commands for the keys.
3. Compile with `gcc g600.c -o g600`.
4. Run with `./g600`.

For command ideas you can look at programs like `ydotool`, `xdo`, `pulseaudio-ctl`, `xclip`, `i3-msg`. You can also run your own scripts.

The program needs privileges to communicate with G600 so typically it'll be started with `sudo`. Alternatively (works on Ubuntu 18.04) you can force the program to run as the `input` group with:

```bash
sudo chown .input g600
sudo chmod g+s g600
```

### Setup Compiled Executable to run on startup
This means that you never have to worry about running this script ever. Your mouse will just work. We can do this by setting up a systemd service.

1. **Create a systemd service file:**
Open a terminal and create a new service file for your executable. You can use nano or any text editor:

```bash
sudo nano /etc/systemd/system/g600.service
```

2. **Add the service configuration:**
Add the following content to the file:

```ini
[Unit]
Description=Run g600 executable
After=network.target

[Service]
ExecStart=/path/to/your/g600
Restart=always
RestartSec=2
User=your-username
WorkingDirectory=/path/to/your/working/directory
Environment="YDOTOOL_SOCKET=/path/to/your/ydotool_socket"

[Install]
WantedBy=multi-user.target
```

Replace /path/to/your/g600 with the actual path to your g600 executable. Replace your-username with your actual username and /path/to/your/working/directory with the working directory if needed.

    ExecStart specifies the command to run your executable.
    Restart=always ensures that the service will be restarted automatically on failure.
    RestartSec=5 sets a delay of 5 seconds before restarting the service after a failure.
    User specifies the user under which the service will run. Replace your-username with the appropriate username.
    WorkingDirectory sets the working directory for the service.

3. **Reload the systemd daemon:**
After creating the service file, reload the systemd daemon to recognize the new service:

```sh
sudo systemctl daemon-reload
```

4. **Enable the service:**
Enable the service so that it starts automatically on boot:

```sh
sudo systemctl enable g600.service
```

5. **Start the service:**
Start the service immediately:

```sh
sudo systemctl start g600.service
```

6. **Check the service status:**
Verify that the service is running correctly:

```sh
sudo systemctl status g600.service
```

This will ensure that your g600 executable runs automatically when your system starts and will be restarted automatically if it fails.
