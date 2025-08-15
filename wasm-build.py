#!/bin/python
import argparse
import atexit
import http.server
import shutil
import signal
import socketserver
import subprocess
import sys
import threading
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


class ReusableTCPServer(socketserver.TCPServer):
    allow_reuse_address = True


def clearDirectory(dirPath: Path):
    for item in dirPath.iterdir():
        if item.is_file():
            item.unlink()
        elif item.is_dir():
            shutil.rmtree(item)


def devServer(path: Path):
    PORT = 8000

    def handle_factory(*args, **kwargs):
        return http.server.SimpleHTTPRequestHandler(
            *args, directory=str(path), **kwargs
        )

    web_url = f"http://localhost:{PORT}/game"
    webbrowser.open_new(web_url)
    try:
        subprocess.run(["termux-open-url", web_url])
    except FileNotFoundError:
        pass

    with ReusableTCPServer(("", PORT), handle_factory) as httpd:
        atexit.register(httpd.server_close)
        print(f"Serving at {web_url}")

        def shutdown_handler(signum, frame):
            print("\nShutting down server...")
            threading.Thread(target=httpd.shutdown, daemon=True).start()

        signal.signal(signal.SIGINT, shutdown_handler)
        signal.signal(signal.SIGTERM, shutdown_handler)

        httpd.serve_forever()


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
