#!/usr/bin/env python3
"""Create a proper ICO file using Python"""

try:
    from PIL import Image

    # Create a simple 32x32 green icon
    img = Image.new('RGBA', (32, 32), color=(0, 200, 0, 255))

    # Save as ICO with multiple sizes for better compatibility
    sizes = [(16, 16), (32, 32), (48, 48), (256, 256)]
    img.save('icon.ico', format='ICO', sizes=sizes)

    print('Created icon.ico with PIL - multi-size')
except ImportError:
    print("PIL not available, using fallback method")

    # Fallback: Create a minimal valid ICO file manually
    import struct

    # Use 48x48 size (commonly supported)
    width, height = 48, 48

    # ICO file header (6 bytes)
    header = struct.pack('<HHH', 0, 1, 1)

    # Create DIB data for 48x48 RGB image
    pixel_size = 4  # RGBA
    row_size = width * pixel_size
    row_padded = ((row_size + 3) // 4) * 4

    # DIB header (BITMAPINFOHEADER)
    dib_header = struct.pack('<IHHIIIIIIII',
        40,           # header size
        width,        # width
        height * 2,   # height * 2 for ICO format (XOR + AND masks)
        1,            # color planes
        32,           # bits per pixel
        0,            # compression (BI_RGB)
        0,            # image size (can be 0 for BI_RGB)
        0,            # x pixels per meter
        0,            # y pixels per meter
        0,            # colors used
        0             # important colors
    )

    # Create XOR mask (color data)
    pixels = b''
    for y in range(height):
        row = b''
        for x in range(width):
            # Create a simple gradient pattern
            g = int(255 * (x + y) / (width + height))
            row += bytes([0, g, 0, 255])  # B, G, R, A
        row += b'\x00' * (row_padded - len(row))
        pixels += row

    # Create AND mask (1 bit per pixel, must be same height as XOR)
    and_mask = b'\x00' * (row_padded * height)

    image_data = dib_header + pixels + and_mask
    image_size = len(image_data)

    # Directory entry (16 bytes)
    entry = struct.pack('<BBBBHHII',
        48,            # width (48)
        48,            # height (48)
        0,             # colors (0 = >8bpp)
        0,             # reserved
        1,             # color planes
        32,            # bits per pixel
        image_size,
        6 + 16         # offset to image data
    )

    ico_data = header + entry + image_data

    with open('icon.ico', 'wb') as f:
        f.write(ico_data)

    print('Created icon.ico - manual method')
