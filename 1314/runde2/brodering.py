w, h, n = map(int, input().split())

result = []

for _ in range(n):
    i, f = input().split()
    i = int(i)
    result += [f] * i

for i in range(h):
    row = result[i * w:i * w + w]
    if i % 2 == 1:
        row = row[::-1]
    print("".join(row))
