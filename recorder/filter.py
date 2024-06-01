#!/usr/bin/python3
import os

d1 = {}
d2 = {}

with open("../radar-results.txt", "r") as file:
    lines = file.readlines()

    for l in lines:
        tokens = l.rstrip().rsplit()
        if tokens[1] != "failed":
            print(l, end="")