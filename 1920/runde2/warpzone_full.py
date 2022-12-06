import math

n = int(input())

neighbours = []
for i in range(n - 1):
    a, b, c = map(int, input().split())
    neighbours.append([
        i + 1,
        a - 1,
        b - 1,
        c - 1,
    ])

class Queue:
    def __init__(self):
        self.front = []
        self.back = []
        
    def push_front(self, value):
        self.front.append(value)
        
    def pop_back(self):
        if not self.back:
            self.move()
                
        return self.back.pop()
        
    def move(self):
        while self.front:
            self.back.append(self.front.pop())
        
    def empty(self):
        return not (bool(self.front) or bool(self.back))

q = Queue()

distance = [math.inf] * n
visited = [False] * n

distance[0] = 0
visited[0] = True
q.push_front(0)

while not q.empty():
    point = q.pop_back()
    
    if point == n - 1:
        print(distance[point] + 1)
        exit()
        
    for neighbour in neighbours[point]:
        if not visited[neighbour]:
            visited[neighbour] = True
            distance[neighbour] = distance[point] + 1
            q.push_front(neighbour)
