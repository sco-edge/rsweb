#!/usr/bin/python3
import os
import sys
import time
import subprocess
import argparse
import id

def truncate_url(url, truncate_path):
    start_index = url.find("://")
    if start_index == -1:
        if not truncate_path:
            return url
        end_index = url.find("/")
        truncated_url = url[:end_index] if end_index > 0 else url
    else:
        if not truncate_path:
            return url[start_index + 3:]
        end_index = url.find("/", start_index + 3)
        truncated_url = url[start_index + 3:end_index] if end_index > 0 else url[start_index + 3:]

    return truncated_url

def load(url, output):
    trace_output = os.path.join(args.output, truncate_url(url, True))
    # if not os.path.exists(output_for_url):
    #     os.makedirs(output_for_url)

    # trial = f"run-0"
    # while os.path.exists(os.path.join(output_for_url, trial)):
    #     (_, number) = trial.rsplit("-", 1)
    #     trial = f"run-{int(number) + 1}"
    # output_trial = os.path.abspath(os.path.join(output_for_url, trial))

    # url = "https://www.amazon.com/JavaScript-Definitive-Most-Used-Programming-Language/dp/1491952024/"
    # url = "https://fastcampus.co.kr/"
    tick = 0
    # commands = [f"mm-webrecord", output_for_url, "../chrome/linux-128.0.6613.84/chrome-linux64/chrome",
    #             "--disable-fre", "--no-default-browser-check", "--no-first-run", "--window-size=1920,1080",
    #             "--ignore-certificate-errors", "--user-data-dir=/tmp/nonexistent$(date +%%s%%N)", url]
    # commands = ["node", "../index.js", url]
    commands = ["mm-webrecord", trace_output, "node", "../record.js", "https://" + truncate_url(url, False)]
    # commands = ["mm-webrecord", trace_output, "node", "../record.js", "https://www.nytimes.com/"]
    # p = subprocess.Popen(commands, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    p = subprocess.Popen(commands)
    # p = subprocess.Popen(commands, shell=True)

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

    if os.path.exists(os.path.join(trace_output, f"{url}.json")):
        return 0
    else:
        return 1
    
if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("target")
    parser.add_argument("--list", "-l", action='store_true')
    parser.add_argument("--output", default='traces')
    parser.add_argument("--timeout", "-t", default=20)

    global args
    args = parser.parse_args()

    global cwd
    cwd = os.getcwd()

    if not args.list:
        print(args.target, end=" ")
        print(load(args.target, args.output))
    else:
        with open(args.target) as file:
            lines = file.readlines()
            for l in lines:
                tokens = l.rstrip().split()
                # print(tokens[0], tokens[1], end=" ")
                # print(load_single(tokens[1]))
    
                print(tokens[0], end=" ")
                print(load(tokens[0], args.output))
