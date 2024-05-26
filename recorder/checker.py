#!/usr/bin/python3
import os

with open("../cloudflare-radar-domains-top-200-20240513-20240520.csv", "r") as file:
    lines = file.readlines()

    for l in lines:
        tokens = l.rstrip().rsplit()
        if os.path.exists(os.path.join("../traces", tokens[0], "run-0", f"{tokens[0]}.json")):
            print(f"{tokens[0]}")