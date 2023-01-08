n, t = map(int, input().split())

houses = [int(input()) for _ in range(n)]
plots = [int(input()) for _ in range(t)]

really_big_number = 1000000000000


class Point:
    position: int
    type: str
    nearest_house: int

    def __init__(self, position: int, type: str, nearest_house: int = really_big_number):
        self.position = position
        self.type = type
        self.nearest_house = nearest_house

    def __lt__(self, rhs):
        return self.position < rhs.position


points = []
points += [Point(position=x, type="house") for x in houses]
points += [Point(position=x, type="plot") for x in plots]
points.sort()

# LTR pass
prev_house = -really_big_number
for point in points:
    if point.type == "house":
        prev_house = point.position

    point.nearest_house = min(point.nearest_house, abs(prev_house - point.position))

# RTL pass
prev_house = really_big_number
for point in reversed(points):
    if point.type == "house":
        prev_house = point.position

    point.nearest_house = min(point.nearest_house, abs(prev_house - point.position))

# Output pass
for point in points:
    if point.type == "plot":
        print(point.nearest_house)
