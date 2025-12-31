# Tauri Desktop App Setup

This document describes how to set up and build the Tauri desktop application for Vibe Kanban.

## Prerequisites

- Rust (latest stable)
- Node.js (>=18)
- pnpm (>=8)
- Tauri CLI prerequisites (platform-specific)

### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

### Windows
```bash
# Install Microsoft C++ Build Tools
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

### Linux
```bash
# See: https://tauri.app/v1/guides/getting-started/prerequisites
```

## Development

### 1. Install Dependencies
```bash
pnpm install
```

### 2. Run Desktop App in Development Mode
```bash
pnpm run tauri:dev
```

This will:
- Start the Vite dev server for the frontend
- Build and run the Tauri desktop app
- Enable hot reloading for both frontend and backend

## Building

### Debug Build
```bash
pnpm run tauri:build:debug
```

Output directory:
- macOS: `tauri/target/debug/bundle/macos/`
- Windows: `tauri/target/debug/bundle/msi/`
- Linux: `tauri/target/debug/bundle/deb/`

### Release Build
```bash
pnpm run tauri:build
```

Output directory:
- macOS: `tauri/target/release/bundle/macos/`
- Windows: `tauri/target/release/bundle/msi/`
- Linux: `tauri/target/release/bundle/deb/`

## Architecture

```
┌─────────────────────────────────────────┐
│     Tauri Desktop Application          │
│  ┌──────────────────┐  ┌──────────────┐│
│  │  Frontend        │  │  Rust        ││
│  │  (React + TS)    │◄─┤  Backend     ││
│  │  - Vite dev      │  │  - Deployment ││
│  │  - Tauri APIs    │  │  - Database   ││
│  │  - IPC Commands  │  │  - Executors  ││
│  └──────────────────┘  └──────────────┘│
└─────────────────────────────────────────┘
```

## Key Components

### Tauri Commands (Rust)

Location: `tauri/src/commands/`

- `deployment.rs` - Deployment initialization and management
- `projects.rs` - Project CRUD operations
- `tasks.rs` - Task CRUD operations
- `executors.rs` - AI executor management
- `filesystem.rs` - File operations
- `config.rs` - Configuration management

### Tauri API Layer (TypeScript)

Location: `frontend/src/utils/tauri.ts`

- `isTauri()` - Check if running in Tauri environment
- `tauriCommands` - Wrapper for all Tauri IPC commands
- `windowControls` - Window management (minimize, maximize, close)
- `getPlatform()` - Platform detection

### API Adapter

Location: `frontend/src/api/adapter.ts`

- Automatically switches between HTTP and Tauri IPC
- Maintains compatibility with existing code
- Transparent to the application logic

## System Integration

### System Tray
- macOS: Menu bar icon with Show/Hide/Quit
- Windows: Tray icon with Show/Quit
- Linux: Tray icon with Show/Quit

### Menu Bar
- macOS: Native menu bar with app, file, edit, view, window, help menus
- Windows/Linux: Basic menu (optional)

### Window Management
- Minimize to tray
- Title bar overlay (macOS)
- Custom window controls

## Configuration

### Tauri Config

Location: `tauri/tauri.conf.json`

Key settings:
- Window size: 1400x900 (min: 1024x768)
- Dev server: `http://localhost:3000`
- Frontend dist: `../frontend/dist`

### Environment Variables

```bash
# Rust log level
RUST_LOG=info

# Analytics
POSTHOG_API_KEY=your_key
POSTHOG_API_ENDPOINT=your_endpoint
```

## Deployment

### Code Signing

#### macOS
```bash
# Set signing identity in tauri.conf.json
"signingIdentity": "Developer ID Application: Your Name"

# Notarize (required for distribution)
xcrun notarytool submit <app.dmg> --apple-id <email> --password <password> --team-id <team-id>
```

#### Windows
```bash
# Set certificate thumbprint in tauri.conf.json
"certificateThumbprint": "your-certificate-thumbprint"
```

### Auto Updates

Tauri supports auto-updates. Configure in `tauri.conf.json`:

```json
{
  "bundle": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.your-domain.com/{{target}}/{{current_version}}"
      ],
      "dialog": true,
      "pubkey": "your-public-key"
    }
  }
}
```

## Troubleshooting

### Frontend not loading
- Ensure Vite dev server is running on port 3000
- Check `tauri.conf.json` `devUrl` configuration

### Rust compilation errors
- Run `cargo check` in the project root
- Ensure all workspace dependencies are available

### Platform-specific issues
- macOS: Check Xcode Command Line Tools installation
- Windows: Verify Microsoft C++ Build Tools
- Linux: Install all required system dependencies

## Performance Tips

1. **Enable release mode for testing**
   ```bash
   pnpm run tauri:build:debug
   ```

2. **Profile with Rust**
   ```bash
   cargo build --profile release
   ```

3. **Monitor memory usage**
   - Use Activity Monitor (macOS)
   - Use Task Manager (Windows)
   - Use htop (Linux)

## Contributing

When modifying Tauri integration:

1. Test all commands in both dev and production builds
2. Verify platform-specific features (tray, menus, etc.)
3. Test auto-update mechanism
4. Check code signing and notarization
5. Update this documentation

## Additional Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Tauri API](https://tauri.app/v1/api/js/)
- [Vite Guide](https://vitejs.dev/guide/)
- [React + Tauri](https://tauri.app/v1/guides/framework/react/)
