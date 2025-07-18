install:
    cargo build --release --package kegtui --target x86_64-apple-darwin
    cargo bundle --release --bin wrapper --format osx --target x86_64-apple-darwin
    mkdir -p /Applications/ethan
    #!/bin/bash
    if [ -d "/Applications/ethan/kegtui.app" ]; then \
        read -p "kegtui.app already exists. Overwrite? (y/N): " -n 1 -r; \
        echo; \
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then \
            echo "Installation cancelled."; \
            exit 1; \
        fi; \
        rm -rf /Applications/ethan/kegtui.app; \
    fi
    mv target/x86_64-apple-darwin/release/bundle/osx/kegtui.app /Applications/ethan/

create_dmg:
    cargo build --release --package kegtui --target x86_64-apple-darwin
    cargo bundle --release --bin wrapper --format osx --target x86_64-apple-darwin
    mkdir -p dmg_contents
    cp -r target/x86_64-apple-darwin/release/bundle/osx/kegtui.app dmg_contents/
    ln -s /Applications dmg_contents/Applications
    hdiutil create -volname "kegtui" -srcfolder dmg_contents -ov -format UDZO kegtui.dmg
    rm -rf dmg_contents
    echo "DMG created: kegtui.dmg"
