n = int(input())

neighbours = []
for i in range(n - 1):
    shortcuts = map(lambda n: int(n) - 1, input().split())
    
    neighbours.append([i + 1] + [next for next in shortcuts if next > i])

cache = [None] * n
def shortest_path(i):
    if cache[i] is not None:
        return cache[i]
    else:
        if i == n - 1:
            cache[i] = 1
        else:
            cache[i] = min(shortest_path(neighbour) + 1 for neighbour in neighbours[i])

        return cache[i]

print(shortest_path(0))
