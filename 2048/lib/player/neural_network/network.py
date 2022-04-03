"""A vanilla neural network"""

from typing import List
from numpy import dot, exp
from numpy.random import randn


class Network:  # pylint: disable=R0903
    """Neural network"""

    def __init__(self, layers: List[int]):
        """
        Args:
            layers (List[int]): Each integer in the list represents a layer. The value of
             the integer represents the amount of nodes in the layer
        """
        self.__layers = layers
        self.__layer_count = len(layers)
        self.__weights = self.__generate_random_weights()
        self.__biases = self.__generate_random_biases()

    def get_output_layer(self, layer: List[int]) -> List[int]:
        """Predicts the output layer of the given input layer
        Args:
            layer (List[int]): The network its input layer
        Returns (List[int]): The network its output layer
        """
        for i in range(self.__layer_count - 1):
            layer = sigmoid(dot(self.__weights[i], layer) + self.__biases[i])[0]
        return layer

    def __generate_random_biases(self) -> List[List[List[int]]]:
        """Generate a random bias for each node in each layer except for the
        input layer (layer 0)
        """
        return [randn(node, 1) for node in self.__layers[1:]]

    def __generate_random_weights(self) -> List[List[List[int]]]:
        """Generate random weights for each node in each layer except for the input layer
        (layer 0). Each node gets a weight for each node in the previous layer.
        """
        return [
            randn(nodes, nodes_prev_layer)
            for nodes_prev_layer, nodes in zip(self.__layers[:-1], self.__layers[1:])
        ]


def sigmoid(inp):
    """The sigmoid function."""
    return 1.0 / (1.0 + exp(-inp))


def main():
    """Manual testing"""
    layers = [4, 3, 2]
    network = Network(layers)
    input_layer = [0, 0, 1, 1]
    print(network.get_output_layer(input_layer))


if __name__ == "__main__":
    main()
