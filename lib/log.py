import json
import time
import os
import shutil
from os import listdir
from os.path import isfile, join

class Logger:
    def __init__(self):
        self.entries = []
        self.savingPath = os.getcwd()+'/logcache'

    def reset(self):
        [os.remove('/'.join([self.savingPath, f])) for f in os.listdir(self.savingPath)]

    def add(self, data):
        self.entries.append(data)

    def openFile(self, gen):
        return json.load(open('/'.join([self.savingPath, gen])))

    def saveAndReset(self, fileName):
        self.reset()
        self.save(fileName)
        self.entries = []

    def save(self, fileName):
        file_ = open(os.path.join(self.savingPath, fileName), 'w')
        file_.write(json.dumps(self.entries))
        file_.close()
