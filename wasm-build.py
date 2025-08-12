#!/bin/python
import argparse
import http.server
import os
import shutil
import socketserver
import subprocess
import sys
import webbrowser
from pathlib import Path

defaultLibName = "luminara"

parser = argparse.ArgumentParser(
    description="Build a Rust Macroquad project for WASM",
    formatter_class=argparse.ArgumentDefaultsHelpFormatter,
)
parser.add_argument(
    "-l",
    "--libName",
    help="Name of library produced by rust",
    type=str,
    default=defaultLibName,
)
parser.add_argument(
    "-r", "--run", help="Start a web server and serve the output", action="store_true"
)
parser.add_argument(
    "-p",
    "--profile",
    help="The cargo profile to use when compiling",
    type=str,
    default="dev",
)
parser.add_argument(
    "-o", "--outputDir", help="Output directory", type=str, default="dist"
)
args = parser.parse_args()


def clearDirectory(dirPath: Path):
    for item in dirPath.iterdir():
        if item.is_file():
            item.unlink()
        elif item.is_dir():
            shutil.rmtree(item)


def devServer(path: Path):
    os.chdir(outPath)
    PORT = 8000
    Handler = http.server.SimpleHTTPRequestHandler
    webbrowser.open_new(f"http:://localhoast:{PORT}/game")
    try:
        subprocess.run(["termux-open-url", f"http://localhost:{PORT}/game"])
    except FileNotFoundError:
        pass

    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"Serving at http://localhost:{PORT}")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("Exiting...")
            httpd.shutdown()


if __name__ == "__main__":
    outPath = Path(args.outputDir)
    outPath.mkdir(parents=True, exist_ok=True)
    clearDirectory(outPath)

    try:
        subprocess.run(
            [
                "cargo",
                "build",
                "--target=wasm32-unknown-unknown",
                "--profile",
                args.profile,
            ],
            check=True,
        )
    except subprocess.CalledProcessError as e:
        print(f"\n\nCargo failed with exit code {e.returncode}")
        print(f"Command: {e.cmd}")
        sys.exit(e.returncode)

    helperPath = Path("wasm_helper")
    assetsPath = Path("assets")

    shutil.copytree(helperPath, outPath, dirs_exist_ok=True)
    shutil.copytree(assetsPath, outPath / "game/assets")

    profilePath = args.profile if not (args.profile == "dev") else "debug"
    libPath = (
        Path("target/wasm32-unknown-unknown/")
        .joinpath(profilePath)
        .joinpath(args.libName + ".wasm")
    )
    shutil.copy2(libPath, outPath / "game/game.wasm")

    if args.run:
        devServer(outPath)
