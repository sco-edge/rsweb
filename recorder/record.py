#!/usr/bin/env python
import os
import sys
import time
import subprocess
import argparse
import pageload

def load_single(url, trace_dir):
    commands = ["mm-webrecord-h3", trace_dir, "./pageload.py", args.target,
                "--dep-output", trace_dir, "--timeout", str(args.timeout)]
    with open(f"output.out", "w") as pl_output:
        subprocess.Popen(commands, stdout=pl_output).communicate()
    
if __name__ == "__main__":
    global cwd
    cwd = os.getcwd()
    
    parser = argparse.ArgumentParser()
    parser.add_argument("target")
    parser.add_argument("--list", "-l", action='store_true')
    parser.add_argument("--output", default=os.path.join(cwd, "..", "traces"))
    parser.add_argument("--timeout", "-t", default=30)

    global args
    args = parser.parse_args()

    output_for_url = os.path.join(args.output, args.target)
    if not os.path.exists(output_for_url):
        os.mkdir(output_for_url)

    trial = f"run-0"
    while os.path.exists(os.path.join(output_for_url, trial)):
        (remained, last) = trial.rsplit("-", 1)
        trial = f"{remained}-{int(last) + 1}"
    output_trial = os.path.join(output_for_url, trial)

    if not args.list:
        load_single(args.target, os.path.abspath(output_trial))
