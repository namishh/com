n =21
is_prime = True

i = 2

while i <= n**0.5:
    if n % i == 0:
        is_prime = False
        break
    i = i + 1

print(is_prime)