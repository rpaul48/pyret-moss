
fun len-of-list<A>(list-name :: List<A>) -> Number:
    doc: "Compute the length of a list"
    # to start, we break down into cases
    cases (List<A>) list-name:
        # in the base case, the list has no length--so 0
        | empty => 0

        # in the non-empty case, the first element counts for 1, and
        # then we need the length of the rest of the list
        | link(f, r) => 1 + len-of-list(r)
    end
end

check "tests for len-of-list":
    len-of-list([list: 1, 5, 0, 8, 13]) is 5
    len-of-list(empty) is 0
    len-of-list([list: 0, 0, 0]) is 3
    len-of-list([list: "test", "one", " ", "two", "three", "four"]) is 6
end