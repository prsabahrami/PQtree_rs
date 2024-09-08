from pqtree import P, Q


def test():
    p = P([1, Q([4, 5]), 3])

    print("Before reverse:\n",p)

    p.reverse()

    print("After:\n",p)


if __name__ == '__main__':
    test()