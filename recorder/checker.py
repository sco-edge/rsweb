#!/usr/bin/python3
import os

d1 = {}
d2 = {}

with open("../radar-passed.csv", "r") as file:
    lines = file.readlines()

    for l in lines:
        tokens = l.rstrip().rsplit()
        if os.path.exists(os.path.join("../../data/traces-1", tokens[0], "run-0", f"{tokens[0]}.json")):    
            if os.path.exists(os.path.join("../../data/traces-2", tokens[0], "run-0", f"{tokens[0]}.json")):
                print(tokens[0])