#!/usr/bin/env python
import os
import sys
import time
import argparse

from selenium import webdriver
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.common.desired_capabilities import DesiredCapabilities
from selenium.webdriver.common.by import By

def load(url, output, timeout):
    print(output)
    cwd = os.getcwd()
    chrome_binary = os.path.join(cwd, "..", "chromes", "125.0.6422.78", "chrome-linux64", "chrome")
    chrome_driver = os.path.join(cwd, "..", "chromes", "125.0.6422.78", "chromedriver-linux64", "chromedriver")
    chrome_extension = os.path.join(cwd, "..", "v2-packed", "depqoe-master.crx")

    service = webdriver.ChromeService(executable_path=chrome_driver)
    #log = open(log_file, "a")

    # Capabilities
    # capabilities = DesiredCapabilities.CHROME
    # capabilities['goog:loggingPrefs'] = {'browser': 'ALL'}

    # Options
    options = webdriver.ChromeOptions()
    options.page_load_strategy = 'none'
    options.binary_location = chrome_binary
    options.add_argument("ignore-certificate-errors")
    options.add_argument("--window-size=1920,1080")

    prefs: dict = {"net.network_prediction_options": 2}
    prefs["download.default_directory"] = output
    # prefs["download.prompt_for_download"] = False
    # prefs["safebrowsing_for_trusted_sources_enabled"] = False
    # prefs["safebrowsing.enabled"] = False
    options.add_experimental_option("prefs", prefs)

    options.add_extension(chrome_extension)

    try:
        # driver = webdriver.Chrome(executable_path=chromium_driver,
        #                             desired_capabilities=capabilities, options=options)
        driver = webdriver.Chrome(service=service, options=options)
        driver.set_page_load_timeout(timeout)

        driver.get("file://" + os.path.join(cwd, "empty.html"))
        script = "body = document.querySelector('body');" \
            + "var element = document.createElement('div');" \
            + "element.id = 'destination';" \
            + "text = document.createTextNode('" + "https://" + url + "');" \
            + "element.appendChild(text);" \
            + "body.append(element);"
        driver.execute_script(script)

        WebDriverWait(driver, timeout=timeout).until(
            lambda d: d.find_element(By.TAG_NAME, "exitSignalSelenium"))
        
    except:
        print(f"{url} error")
        # print(str(sys.exc_info()[0]), file=sys.stderr)
        # print("\n", file=sys.stderr)
    else:
        print(f"{url}")
    # driver.quit()

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("url")
    parser.add_argument("--output", default=os.path.join(os.getcwd(), "output"))
    parser.add_argument("--timeout", "-t", default=30)

    args = parser.parse_args()

    load(args.url, args.output, args.timeout)