# Packaging Guide

This document describes how to package Rise RTS for distribution on different platforms.

## Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- Platform-specific tools (see below)

## Windows

### Option 1: Simple ZIP Bundle

```powershell
# Build release
cargo build --release -p client

# Create distribution folder
mkdir dist\windows
copy target\release\client.exe dist\windows\rise-rts.exe
xcopy /E assets dist\windows\assets\

# Create ZIP
Compress-Archive -Path dist\windows\* -DestinationPath rise-rts-windows.zip
```

### Option 2: Installer with Inno Setup

1. Install [Inno Setup](https://jrsoftware.org/isinfo.php)
2. Create an installer script `installer.iss`:

```iss
[Setup]
AppName=Rise RTS
AppVersion=0.1.0
DefaultDirName={autopf}\Rise RTS
DefaultGroupName=Rise RTS
OutputDir=dist
OutputBaseFilename=rise-rts-setup

[Files]
Source: "target\release\client.exe"; DestDir: "{app}"; DestName: "rise-rts.exe"
Source: "assets\*"; DestDir: "{app}\assets"; Flags: recursesubdirs

[Icons]
Name: "{group}\Rise RTS"; Filename: "{app}\rise-rts.exe"
Name: "{commondesktop}\Rise RTS"; Filename: "{app}\rise-rts.exe"
```

3. Build: `iscc installer.iss`

### Option 3: MSIX Package (Windows Store)

TODO: Add MSIX packaging instructions using `cargo-msix` or manual manifest.

## macOS

### Option 1: App Bundle

```bash
# Build release
cargo build --release -p client

# Create app bundle structure
mkdir -p "Rise RTS.app/Contents/MacOS"
mkdir -p "Rise RTS.app/Contents/Resources"

# Copy binary
cp target/release/client "Rise RTS.app/Contents/MacOS/rise-rts"

# Copy assets
cp -r assets "Rise RTS.app/Contents/Resources/"

# Create Info.plist
cat > "Rise RTS.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>rise-rts</string>
    <key>CFBundleIdentifier</key>
    <string>com.rise-rts.game</string>
    <key>CFBundleName</key>
    <string>Rise RTS</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF
```

### Option 2: DMG Installer

```bash
# After creating app bundle
hdiutil create -volname "Rise RTS" -srcfolder "Rise RTS.app" -ov -format UDZO rise-rts.dmg
```

### Code Signing (Required for Distribution)

```bash
# Sign the app (requires Apple Developer account)
codesign --deep --force --verify --verbose \
    --sign "Developer ID Application: Your Name (TEAM_ID)" \
    "Rise RTS.app"

# Notarize (required for Gatekeeper)
xcrun notarytool submit rise-rts.dmg \
    --apple-id "your@email.com" \
    --team-id "TEAM_ID" \
    --password "app-specific-password" \
    --wait
```

## Linux

### Option 1: AppImage

1. Install [appimagetool](https://github.com/AppImage/AppImageKit)

```bash
# Build release
cargo build --release -p client

# Create AppDir structure
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps

# Copy files
cp target/release/client AppDir/usr/bin/rise-rts
cp -r assets AppDir/usr/share/rise-rts/

# Create desktop file
cat > AppDir/usr/share/applications/rise-rts.desktop << EOF
[Desktop Entry]
Type=Application
Name=Rise RTS
Exec=rise-rts
Icon=rise-rts
Categories=Game;StrategyGame;
EOF

# Create AppRun
cat > AppDir/AppRun << EOF
#!/bin/bash
HERE="\$(dirname "\$(readlink -f "\${0}")")"
export PATH="\$HERE/usr/bin:\$PATH"
exec rise-rts "\$@"
EOF
chmod +x AppDir/AppRun

# Build AppImage
appimagetool AppDir rise-rts-x86_64.AppImage
```

### Option 2: Flatpak

TODO: Add Flatpak manifest and build instructions.

### Option 3: Snap

TODO: Add snapcraft.yaml and build instructions.

### Option 4: Debian Package

```bash
# Create package structure
mkdir -p rise-rts_0.1.0-1_amd64/DEBIAN
mkdir -p rise-rts_0.1.0-1_amd64/usr/bin
mkdir -p rise-rts_0.1.0-1_amd64/usr/share/rise-rts

# Copy files
cp target/release/client rise-rts_0.1.0-1_amd64/usr/bin/rise-rts
cp -r assets/* rise-rts_0.1.0-1_amd64/usr/share/rise-rts/

# Create control file
cat > rise-rts_0.1.0-1_amd64/DEBIAN/control << EOF
Package: rise-rts
Version: 0.1.0-1
Section: games
Priority: optional
Architecture: amd64
Depends: libasound2, libudev1
Maintainer: Your Name <your@email.com>
Description: Rise of Nations-inspired RTS game
 A real-time strategy game built with Rust and Bevy.
EOF

# Build package
dpkg-deb --build rise-rts_0.1.0-1_amd64
```

## Cross-Compilation

### Using cross

```bash
# Install cross
cargo install cross

# Build for different targets
cross build --release -p client --target x86_64-pc-windows-gnu
cross build --release -p client --target x86_64-unknown-linux-gnu
cross build --release -p client --target x86_64-apple-darwin
```

## Recommended Tools

- **cargo-bundle**: Automated macOS/Windows bundling
- **cargo-deb**: Debian package generation
- **cargo-wix**: Windows MSI installer generation
- **cargo-appimage**: AppImage generation

```bash
# Install bundling tools
cargo install cargo-bundle
cargo install cargo-deb
```

## CI/CD Integration

See `.github/workflows/ci.yml` for automated builds on GitHub Actions.

For release builds, consider using GitHub Releases with artifacts uploaded from CI.
