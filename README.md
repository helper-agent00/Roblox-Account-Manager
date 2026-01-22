# Nexus Account Manager

A lightweight Roblox account manager built in Rust. Manage multiple accounts, launch games, browse servers, and more.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)

## Features

- **Account Management** - Store and organize multiple Roblox accounts
- **Quick Launch** - Launch any account into games with one click
- **Server Browser** - Browse public servers with player counts and ping
- **Multi-Instance** - Run multiple Roblox clients simultaneously
- **Cookie Import** - Import accounts via security cookie (drag & drop supported)
- **Presence Tracking** - See which accounts are online/in-game
- **Favorite Games** - Quick access to your most played games
- **System Tray** - Minimize to tray to keep it out of the way

## Getting Started

### Adding Accounts

There's a few ways to add accounts:

1. **Browser Login** - Go to Add Account tab, click "Login via Browser". This opens a browser window where you can log into Roblox normally. The cookie gets captured automatically.

2. **Cookie Import** - If you already have a `.ROBLOSECURITY` cookie, paste it in the Import Cookie tab. You can also just drag and drop a text file containing the cookie onto the window.

3. **Auto-Find** - The app can scan your browsers for existing Roblox sessions. Hit "Find Cookies" in the Add Account tab.

### Multi-Instance

By default, Roblox only allows one client at a time. To run multiple accounts:

1. Go to **Settings** tab
2. Enable **"Multi-Instance Mode"**
3. Launch your accounts - each one opens in its own Roblox client

Note: This uses a mutex unlock method. Works on Windows.

### Minimize to Tray

If you want the app to hide to the system tray instead of sitting in your taskbar:

1. Go to **Settings** tab  
2. Enable **"Minimize to tray"**
3. Click the `_` button in the top-right header to minimize
4. Right-click the tray icon → "Show" to bring it back
5. Right-click → "Quit" to close completely

## Building

```bash
cargo build --release
```

Requires an `icon.ico` in the project root for the tray icon.

## Disclaimer

Use at your own risk. This tool interacts with Roblox in unofficial ways. Don't do anything that violates Roblox ToS.

---

Questions? Join the [Nexus Underground](https://discord.gg/U8ehekqN64) Discord.
