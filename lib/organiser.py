import json
import pprint
import random
from statistics import mean

class Organiser:
    def __init__(self):
        self.counter  = {'left': 0.00, 'right': 0.00, 'down': 0.00, 'up': 0.00}
        self.direction_percentages = {'left': 25.00, 'right': 25.00, 'down': 25.00, 'up': 25.00}

    def movement_counter(self, movement):
        self.counter[movement] += 1

    def percentage_counter(self):
        total_movements = sum(self.counter.values())
        for cdir in self.counter:
            self.direction_percentages[cdir] = (self.counter[cdir] / total_movements) * 100
        return self.direction_percentages

    def sort_scores(self, data):
        unsorted_data = [game for generation in data for game in generation]
        return sorted(data, reverse = True, key = (lambda game: game['score']))

    def set_movement_chance(self, data, last = 10):
        move_percentage = {'left': [], 'right': [],'down': [], 'up': []}
        for game in self.sort_scores(data)[0:last]:
            for move in move_percentage:
                move_percentage[move].append(game[move])
        for move in move_percentage:
            move_percentage[move] = mean(move_percentage[move])
        return move_percentage
