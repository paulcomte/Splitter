#!/bin/sh

# loop through all png files in the current directory
for file in *.png
do
    # check if the file is a regular file
    if [[ -f "$file" ]]; then
        # execute the convert command with the xterm-colors.gpl palette file
        convert "$file" -remap ../xterm-colors.gpl "$file"
    fi
done

