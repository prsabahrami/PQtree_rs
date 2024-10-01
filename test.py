from pqtree import P, Q


def basic_test():
    print("Test 1")
    p = P([1, Q([4, 5]), 3])

    print("Before reverse:\n",p)

    p.reverse()

    print("After:\n",p)

    print(p.number_of_children())

    print(type(p.get_children()[2]))


def flatten_test():
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


def ordering_test():
    p = Q([[1,2], [2,3], P([[2,4], [2,8], [2,9]])])

    print("Ordering:", p.ordering())

def cardinality_test():
    p = P([1,2,3,4,5])

    q = Q([1,2,3,4,5])

    print("Cardinality:", p.cardinality())

    print("Cardinality:", q.cardinality())


    p = P([1, 2, 3, Q([4, 5, 6]), P([7, 8, 9])])

    print("Cardinality:", p.cardinality())

if __name__ == '__main__':
    basic_test()
    flatten_test()
    ordering_test()
    cardinality_test()