n, m = map(int, input().split())

for _ in range(m):
    x = int(input())
    if x < n:
        print("FOR LITE")
    elif x > n:
        print("FOR MYE")
    else:
        print("RIKTIG")
