#!/usr/bin/env python
import os
import sys
import time
import subprocess
import argparse
import pageload

if __name__ == "__main__":
    global cwd
    cwd = os.getcwd()
    
    parser = argparse.ArgumentParser()
    parser.add_argument("target")
    parser.add_argument("--list", "-l", action='store_true')
    parser.add_argument("--deps", default=os.path.join(cwd, "..", "deps"))
    parser.add_argument("--traces", default=os.path.join(cwd, "..", "traces"))
    parser.add_argument("--timeout", "-t", default=30)

    global args
    args = parser.parse_args()

    trace_dir = os.path.join(args.traces, args.target)

    commands = ["mm-webrecord-h3", "../traces/google.com-3/", "./pageload.py", args.target,
                "--output", os.path.abspath(args.deps), "--timeout", str(args.timeout)]
    with open(f"output.out", "w") as pl_output:
        p = subprocess.Popen(commands, stdout=pl_output).communicate()