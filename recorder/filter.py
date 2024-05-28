#!/usr/bin/python3
import os

d1 = {}
d2 = {}

with open("../radar-results.txt", "r") as file:
    lines = file.readlines()

    for l in lines:
        tokens = l.rstrip().rsplit()
        print(tokens[0])