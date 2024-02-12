from sys import argv
from utils.list import Dim
from collections.vector import DynamicVector

fn is_prime(x: Int) -> Bool:
    var ret: Bool = True
    var limit: Int = (x ** 0.5).to_int() + 1
    var check: Int = 2
    while check <= limit:
        if x % check == 0:
            ret = False
            break
        else:
            check += 1
    return ret


fn main():
    var cli_args = argv()
    if len(argv()) < 2:
        print("Usage: primerator <num>")
    else: 
        try:
            let prime_count: Int = atol(argv()[1]) 
            
            var primes = DynamicVector[Int](prime_count)
            primes.push_back(2)
            var to_test: Int = 3
            while prime_count > primes.__len__():
                if(is_prime(to_test)):
                    primes.push_back(to_test)
                to_test += 2
            print(primes[0])
            print(primes[prime_count - 1])
        except:
            print("Error: Argument is not an integer")
