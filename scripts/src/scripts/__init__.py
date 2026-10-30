from pathlib import Path

from rembg import remove

from PIL import Image

def main() -> None:
    process()
    # print("Hello from scripts!")

def process():
    path = get_path()
    input = "download.jpg"
    out = remove(Image.open(path/input))
    output = "down.png"
    out.save(path/output)
    # pass

def get_path():
    path = Path("C:/Users/User/Projects/Valan/scripts/images/")
    path.mkdir(parents=True,exist_ok=True)
    return path