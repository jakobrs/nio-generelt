import queue

n = int(input())

pq = queue.PriorityQueue()
total = 0
removed = 0

for _ in range(n):
    x = int(input())
    total += x
    pq.put(x)
    
    while total < 0:
        total -= pq.get()
        removed += 1

print(removed)
