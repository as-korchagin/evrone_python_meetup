import numpy as np
from haversine.haversine import haversine_m

addresses = (
    (37.4715, 55.6168),
    (37.5258, 55.6739),
    (37.5400, 55.6712),
    (37.5402, 55.6700),
    (37.5305, 55.6740),
    (37.5422, 55.6726),
    (37.5478, 55.6655),
    (37.5476, 55.6654),
    (37.5592, 55.6714),
    (37.5648, 55.6739),
    (37.5982, 55.6731),
    (37.6009, 55.6778),
    (37.5939, 55.6821),
    (37.4715, 55.6168),
)


def generate_matrix():
    m_len = len(addresses)
    matrix = np.zeros((m_len, m_len))
    for row in range(1, m_len):  # matrix[0, 0] == 0 anyway
        for column in range(0, row):
            matrix[row][column] = matrix[column][row] = haversine_m(
                *addresses[row],
                *addresses[column],
            )
    return matrix
