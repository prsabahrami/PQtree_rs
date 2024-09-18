from pqtree import P, Q


def test1():
    print("Test 1")
    p = P([1, Q([4, 5]), 3])

    print("Before reverse:\n",p)

    p.reverse()

    print("After:\n",p)

    print(p.number_of_children())

    print(type(p.get_children()[2]))


def test2():
    print("Test 2")
    p = P([P([1, 2, 3, Q([4, 5])])])

    print("Not flattened:",p)
    print("Flattened", p.flatten())


    q = Q([p, 6, 7])

    print("Not flattened:",q)
    print("Flattened", q.flatten())

    t = q.flatten()

    t.reverse()

    print(t.get_children())

if __name__ == '__main__':
    test1()
    test2()