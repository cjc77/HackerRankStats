import math

P_BOY = 1.09 / 2.09
P_GIRL = 1 / 2.09
N = 6
x = 3


#P_BOY = 1/2
#N = 10
#x = 5


def binom(x, n, p):
    q = 1 - p
    n_choose_x = math.factorial(n)/(math.factorial(x) * math.factorial(n-x))
    p_to_x = math.pow(p, x)
    q_to_diff_n_x = pow(q, n-x)
    return n_choose_x * p_to_x * q_to_diff_n_x


def get_odds():
    result = 0
    for i in range(x, N + 1):
        result += binom(i, N, P_BOY)
    return round(result, 3)





def main():
    result = get_odds()
    print(result)


if __name__=="__main__":
    main()
