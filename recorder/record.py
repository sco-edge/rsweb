#!/usr/bin/env python
import os
import sys
import time
import subprocess
import argparse
import pageload

def load_single(url):
    output_for_url = os.path.join(args.output, url)
    if not os.path.exists(output_for_url):
        os.makedirs(output_for_url)

    trial = f"run-0"
    while os.path.exists(os.path.join(output_for_url, trial)):
        (_, number) = trial.rsplit("-", 1)
        trial = f"run-{int(number) + 1}"
    output_trial = os.path.abspath(os.path.join(output_for_url, trial))
    
    commands = ["mm-webrecord-h3", output_trial, "./pageload.py", url,
                "--dep-output", output_trial, "--timeout", str(args.timeout)]
    p = subprocess.Popen(commands, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        out, err = p.communicate(timeout=args.timeout)
    except:
        p.kill()
        first_commands = ["ps", "-eo", "pid,args"]
        second_commands = ["awk", '/chrome/ {print $1}']
        third_commands = ["xargs", "kill", "-9"]
        p1 = subprocess.Popen(first_commands, stdout=subprocess.PIPE)
        p2 = subprocess.Popen(second_commands, stdin=p1.stdout, stdout=subprocess.PIPE, text=True)
        p3 = subprocess.Popen(third_commands, stdin=p2.stdout, stdout=subprocess.PIPE, text=True)
        p1.stdout.close()  # Allow p1 to receive a SIGPIPE if p2 exits.
        output = p3.communicate()[0]

        return 2

    if os.path.exists(os.path.join(output_trial, f"{url}.json")):
        return 0
    else:
        return 1
    
if __name__ == "__main__":
    global cwd
    cwd = os.getcwd()
    
    parser = argparse.ArgumentParser()
    parser.add_argument("target")
    parser.add_argument("--list", "-l", action='store_true')
    parser.add_argument("--output", default=os.path.join(cwd, "..", "traces"))
    parser.add_argument("--timeout", "-t", default=20)

    global args
    args = parser.parse_args()

    if not args.list:
        print(args.target, end=" ")
        print(load_single(args.target))
    else:
        with open(args.target) as file:
            lines = file.readlines()
            for l in lines:
                tokens = l.rstrip().split()
                # print(tokens[0], tokens[1], end=" ")
                # print(load_single(tokens[1]))
    
                print(tokens[0], end=" ")
                print(load_single(tokens[0]))
