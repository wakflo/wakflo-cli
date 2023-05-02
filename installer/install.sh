echo Downloading Wakflo installer...

# Download the installer
if [[ "$OSTYPE" =~ ^darwin ]]; then
    curl https://cdn.wakflo.ai/wakflo/cli/bin/installer-macos.bin -o installer.bin
fi

if [[ "$OSTYPE" =~ ^linux ]]; then
    curl https://cdn.wakflo.ai/wakflo/cli/bin/installer-linux.bin -o installer.bin
fi

# Make it an executable
chmod +x installer.bin
# Run the installer
./installer.bin