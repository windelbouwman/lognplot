import base64

with open("icon.png", "rb") as f:
    png_data = f.read()

print("icon.png:")
print("--------------")
print(base64.encodebytes(png_data).decode("ascii"))
print("--------------")
