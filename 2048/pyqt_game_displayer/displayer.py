#!/usr/bin/env python3
"""The executable script"""

from src.game_displayer.main_window import run_main_window
from argparse import ArgumentParser


def main():
    """Main script of the program"""
    parser = ArgumentParser()
    parser.add_argument("--path", required=True, help="Path to the json result data")
    args = parser.parse_args()
    run_main_window(args.path)


if __name__ == "__main__":
    main()
