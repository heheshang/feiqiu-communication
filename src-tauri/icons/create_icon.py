import struct

# ICO file header (6 bytes)
header = struct.pack('<HHH', 0, 1, 1)

# Create a minimal 32x32 RGBA bitmap
width, height = 32, 32
pixel_size = 4  # RGBA
row_size = width * pixel_size
row_padded = ((row_size + 3) // 4) * 4

# DIB header (40 bytes) - BITMAPINFOHEADER
dib_header = struct.pack('<IHHIIIIIIII',
    40,           # header size
    width,        # width
    height * 2,   # height * 2 for ICO format
    1,            # color planes
    32,           # bits per pixel
    0,            # compression
    0,            # image size
    0,            # x pixels per meter
    0,            # y pixels per meter
    0,            # colors used
    0             # important colors
)

# Create pixel data (a simple green square)
pixels = b''
for y in range(height):
    row = b''
    for x in range(width):
        row += bytes([0, 200, 0, 255])  # B, G, R, A
    row += b'\x00' * (row_padded - len(row))
    pixels += row

# AND mask (1 bit per pixel)
and_mask = b'\x00' * (row_padded * height)

image_data = dib_header + pixels + and_mask
image_size = len(image_data)

# Directory entry
entry = struct.pack('<BBBBHHII',
    32,     # width
    32,     # height
    0,      # colors
    0,      # reserved
    1,      # color planes
    32,     # bits per pixel
    image_size,
    6 + 16  # offset
)

ico_data = header + entry + image_data

with open('icon.ico', 'wb') as f:
    f.write(ico_data)

print('Created icon.ico')
