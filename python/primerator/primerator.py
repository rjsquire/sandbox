import sys

def is_prime(x):
    ret = True
    limit = int(x ** 0.5) + 1
    check = 2
    while check <= limit: 
        if x % check == 0 or check > limit:
            ret = False
            break
        else:
            check += 1
    return ret

# import pdb; pdb.set_trace()
if len(sys.argv) < 2:
    print("Usage: primerator <num>")
    exit()

arg = sys.argv[1]
if arg.isdigit():
    prime_count = int(arg)
else:
    print("Error: Argument is not an integer.") 
    exit()

primes = []
primes.append(2)
to_test = 3
while prime_count > len(primes):
    if is_prime(to_test):
        primes.append(to_test)
    to_test += 2

print(primes)

