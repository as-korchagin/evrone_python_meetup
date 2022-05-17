# -*- coding: utf-8 -*-

# external
import numpy as np
from numpy.random import uniform
from numba import njit


@njit
def path_durations(matrix, paths):
    durations = np.empty(paths.shape[0])
    for path_idx, ant_path in enumerate(paths):
        p_duration = 0
        for step_idx in range(1, len(paths)):
            p_duration += matrix[ant_path[step_idx - 1]][ant_path[step_idx]]
        durations[path_idx] = p_duration
    return durations


@njit
def generate_paths(
    matrix,
    matrix_size,
    places,
    pheromone,
    alpha,
    heuristic,
    beta,
):
    ants_path = np.zeros(matrix.shape, dtype='i2') - 1  # init
    ants_path[:, 0] = 0  # first point is always the same
    ants_path[:, matrix_size - 1] = matrix_size - 1  # last point is always the same
    # Don't take first and last points
    for place_idx in range(1, matrix_size - 1):
        for current_ant in range(matrix_size):
            unvisited, current_unvisited = np.zeros(matrix_size, 'u1'), 0
            for point in places:
                # Numba doesn't work with numpy.isin
                if point not in ants_path[current_ant]:
                    unvisited[current_unvisited] = point
                    current_unvisited += 1
            unvisited = unvisited[0: current_unvisited]
            if len(unvisited) == 1:
                ants_path[current_ant][place_idx] = unvisited[0]
                continue
            p_total = 0
            for u_point in unvisited:
                p_total += (
                    pheromone[ants_path[current_ant][place_idx - 1]][u_point] ** alpha *
                    heuristic[ants_path[current_ant][place_idx - 1]][u_point] ** beta
                )
            p_by_unvisited = np.zeros(unvisited.shape[0])  # probability by unvisited points
            for p_idx in range(p_by_unvisited.shape[0]):
                p_by_unvisited[p_idx] = (
                    pheromone[ants_path[current_ant][place_idx - 1]][
                        unvisited[p_idx]] ** alpha *
                    heuristic[ants_path[current_ant][place_idx - 1]][unvisited[p_idx]] ** beta
                ) / p_total
            cumsum = p_by_unvisited.cumsum()
            p_random = uniform(0, max(cumsum))
            # Setting next place to go
            ants_path[current_ant][place_idx] = unvisited[np.where(cumsum >= p_random)[0][0]]
    return ants_path


@njit()
def optimize(matrix, iterations):
    alpha, beta = 1, 1  # todo: mutable
    phe_eva_rate = 0.3
    pheromone = np.ones(matrix.shape)
    matrix_size = matrix.shape[0]
    places = np.arange(0, matrix_size)
    heuristic = 1 / (np.eye(matrix_size) + matrix) - np.eye(matrix_size)
    min_duration = np.sum(matrix)
    min_route = None

    for _ in range(iterations):
        ants_path = generate_paths(matrix, matrix_size, places, pheromone, alpha, heuristic, beta)
        pheromone *= (1 - phe_eva_rate)
        durations = path_durations(matrix, ants_path)
        for path_idx, ant_route in enumerate(ants_path):
            delta_tau = 1 / durations[path_idx]
            for place_idx in range(1, matrix_size - 1):
                pheromone[ant_route[place_idx - 1]][ant_route[place_idx]] += delta_tau
        min_idx = np.argmin(durations)
        if durations[min_idx] < min_duration:
            min_duration = durations[min_idx]
            min_route = ants_path[min_idx]
    
    return min_route
