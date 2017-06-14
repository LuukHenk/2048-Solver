import json
import time
import os
import shutil
from os import listdir
from os.path import isfile, join

class Logger:
    def __init__(self):
        self.entries = []
        self.saving_path = '/mnt/c/Users/luukp/Desktop/Programmeren/Python/ai/logcache'

    def reset(self):
        [os.remove('/'.join([self.saving_path, f])) for f in os.listdir(self.saving_path)]

    def add(self, data):
        self.entries.append(data)

    def load_logs(self):
        file_names = [f for f in os.listdir(self.saving_path)]
        data       = [json.load(open('/'.join([self.saving_path, f]))) for f in file_names]
        return list(map(lambda json: json[0], data))

    def save_and_reset(self, file_name):
        self.save(file_name)
        self.entries = []

    def save(self, file_name):
        file_ = open(os.path.join(self.saving_path, file_name), 'w')
        file_.write(json.dumps(self.entries))
        file_.close()
