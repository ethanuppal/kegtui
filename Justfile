install:
    cargo build --release --package kegtui --target x86_64-apple-darwin
    cargo bundle --release --bin wrapper --format osx --target x86_64-apple-darwin
    mkdir -p /Applications/ethan
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
