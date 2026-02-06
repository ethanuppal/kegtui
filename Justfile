install:
    cargo build --release --package kegtui --target x86_64-apple-darwin
    cargo bundle --release --bin wrapper --format osx --target x86_64-apple-darwin
    mkdir -p /Applications/$USER
    #!/bin/bash
    if [ -d "/Applications/$USER/kegtui.app" ]; then \
        read -p "kegtui.app already exists. Overwrite? (y/N): " -n 1 -r; \
        echo; \
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then \
            echo "Installation cancelled."; \
            exit 1; \
        fi; \
        rm -rf /Applications/$USER/kegtui.app; \
    fi
    mv target/x86_64-apple-darwin/release/bundle/osx/kegtui.app /Applications/$USER/

uninstall:
    if [ -d "/Applications/$USER/kegtui.app" ]; then \
        read -p "Are you sure you want to uninstall? (y/N): " -n 1 -r; \
        echo; \
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then \
            echo "Uninstallation cancelled."; \
            exit 1; \
        fi; \
        rm -rf /Applications/$USER/kegtui.app; \
    else \
        echo "kegtui.app not installed to /Applications/$USER"; \
        exit 1; \
    fi

create_dmg:
    cargo build --release --package kegtui --target x86_64-apple-darwin
    cargo bundle --release --bin wrapper --format osx --target x86_64-apple-darwin
    mkdir -p dmg_contents
    cp -r target/x86_64-apple-darwin/release/bundle/osx/kegtui.app dmg_contents/
    ln -s /Applications dmg_contents/Applications
    hdiutil create -volname "kegtui" -srcfolder dmg_contents -ov -format UDZO kegtui.dmg
    rm -rf dmg_contents
    echo "DMG created: kegtui.dmg"
