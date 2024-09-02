#!/usr/bin/python3
import random
import string

def gen_id():
    return ''.join(random.choices(string.ascii_lowercase, k=4)) + ''.join(random.choices(string.digits, k=4))

print(gen_id())