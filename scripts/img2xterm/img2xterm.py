#!/usr/bin/env python

"""
    Short script to convert an image to xterm-256 equivalent
"""

import struct

from PIL import Image
from sys import exit, stderr, argv

# https://gist.github.com/MicahElliott/719710
# Default color levels for the color cube
CUBELEVELS = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff]
# Generate a list of midpoints of the above list
SNAPS = [(x+y)/2 for x, y in zip(CUBELEVELS, [0]+CUBELEVELS)][1:]

def rgb2short(r, g, b):
    """
        Converts RGB values to the nearest equivalent xterm-256 color.
    """

    # Using list of snap points, convert RGB value to cube indexes
    r, g, b = map(
        lambda x: len(tuple(s for s in SNAPS if s<x)),
        (r, g, b)
    )
    
    # Simple colorcube transform
    return r*36 + g*6 + b + 16

def main():
    av = argv[1:]
    
    if len(av) != 2:
        exit(1)

    (src, dest) = av
    
    try:
        img = Image.open(src)
        fd = open(dest, "wb+")
    except Exception as error:
        print(error, file=stderr)
        exit(1)

    # Get RGB tuples    
    data = img.getdata()

    for rgb in data:
        if len(rgb) != 3:
            continue
        
        fd.write(struct.pack(">B", rgb2short(*rgb)))

if __name__ == "__main__":
    main()
