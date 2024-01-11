<?php
// echo "primerator";
// print( var_dump($argv));
// print(count($argv));
if (count($argv) < 2) {
    echo "Usage: primerator <num>";
    exit(1);
}

$arg = $argv[1];
if (is_numeric($arg)) {
    $prime_count = (int)$arg;
}
else {
    echo "Argument must be a number";
    exit(1);
}

$primes = [];
$primes[] = 2;
$to_test = 3;
while ($prime_count > count($primes)) {
    if (is_prime($to_test)) {
        $primes[] = $to_test;
    }
    $to_test += 2;
}
print_r($primes);

function is_prime($x) {
    $ret = true;
    $limit = ceil(sqrt($x));
    // echo "X {$x} :: Limit {$limit} \n";
    $check = 2;
    while ($check <= $limit) {
        if ($x % $check == 0) {
            $ret = false;
            break;
        }
        else {
            $check ++;
        }
    }
    return $ret;
}