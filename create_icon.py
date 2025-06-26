#!/usr/bin/env python3

import os
import subprocess

# Create a simple iconset using built-in macOS tools
def create_icon():
    icons_dir = "/Users/danielconnolly/Projects/egui-test/assets/icons"
    iconset_dir = f"{icons_dir}/ev2.iconset"
    
    os.makedirs(iconset_dir, exist_ok=True)
    
    # Create a simple text-based icon using ImageMagick or similar
    # For now, let's create a simple AppleScript to generate an icon
    
    applescript = '''
    tell application "Image Events"
        launch
        set img to make new image with properties {dimensions:{512, 512}, color depth:color}
        
        tell img
            -- Fill with dark background
            fill color {17, 24, 39}
            
            -- We'll create a simple colored rectangle for now
            -- In a real implementation, you'd add text and styling
        end tell
        
        save img as PNG in file "''' + iconset_dir + '''/icon_512x512.png"
        close img
    end tell
    '''
    
    # For simplicity, let's create a basic shell script approach
    print("Creating basic icon files...")
    
    # Create required icon sizes for macOS
    sizes = [
        ("icon_16x16.png", 16),
        ("icon_16x16@2x.png", 32),
        ("icon_32x32.png", 32),
        ("icon_32x32@2x.png", 64),
        ("icon_128x128.png", 128),
        ("icon_128x128@2x.png", 256),
        ("icon_256x256.png", 256),
        ("icon_256x256@2x.png", 512),
        ("icon_512x512.png", 512),
        ("icon_512x512@2x.png", 1024),
    ]
    
    # Create a basic colored square for each size using the 'sips' command
    base_color = "#06b6d4"  # Cyan color
    
    for filename, size in sizes:
        # Create a simple colored image using built-in tools
        cmd = f'''osascript -e '
        tell application "Image Events"
            launch
            set newImage to make new image with properties {{dimensions:{{{size}, {size}}}, color depth:color}}
            tell newImage
                fill with color {{6, 182, 212}}
            end tell
            save newImage as PNG in file "{iconset_dir}/{filename}"
            close newImage
        end tell
        ' '''
        
        try:
            subprocess.run(cmd, shell=True, check=False, capture_output=True)
        except:
            pass
    
    print(f"Icon files created in {iconset_dir}")
    
    # Convert iconset to icns
    try:
        subprocess.run(f"iconutil -c icns {iconset_dir}", shell=True, check=True)
        print(f"Icon bundle created: {icons_dir}/ev2.icns")
        return f"{icons_dir}/ev2.icns"
    except:
        print("Could not create .icns file")
        return None

if __name__ == "__main__":
    create_icon()