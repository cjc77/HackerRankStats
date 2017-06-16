import math

P_REJECT = 0.12
N = 10
x = 2


def binom(temp_x, n, temp_p):
    q = 1 - temp_p
    n_choose_temp_x = math.factorial(n) / (math.factorial(temp_x) * math.factorial(n - temp_x))
    p_to_temp_x = math.pow(temp_p, temp_x)
    q_to_diff_ntemp_x = math.pow(q, n - temp_x)
    return n_choose_temp_x * p_to_temp_x * q_to_diff_ntemp_x

def at_most_2():
    result = 0
    for i in range(0, x + 1):
        result += binom(i, N, P_REJECT)
    return round(result, 3)

def at_least_2():
    result = 0
    for i in range(x, N + 1):
        result += binom(i, N, P_REJECT)
    return round(result, 3)

def main():
    at_most = at_most_2()
    at_least = at_least_2()
    print(at_most, at_least, sep='\n')


if __name__ == "__main__":
    main()
