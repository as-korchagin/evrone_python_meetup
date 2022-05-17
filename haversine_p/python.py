import math

EARTH_RADIUS_M = 6372800


def haversine_m(
    lon1_deg: float,
    lat1_deg: float,
    lon2_deg: float,
    lat2_deg: float,
):

    lon1rad = lon1_deg * math.pi / 180
    lon2rad = lon2_deg * math.pi / 180
    lat1rad = lat1_deg * math.pi / 180
    lat2rad = lat2_deg * math.pi / 180

    return math.asin(
        math.sqrt(
            math.pow(math.sin((lat2rad - lat1rad) / 2), 2) + math.cos(lat1rad) *
            math.cos(lat2rad) * math.pow(math.sin((lon2rad - lon1rad) / 2), 2)
        )
    ) * 2 * EARTH_RADIUS_M
