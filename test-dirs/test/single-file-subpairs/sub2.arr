

# List length checking function:
fun list-length(i):
    doc: "Compute the property of the list that is its length"
    cases (List) i:
        | empty => 0
        | link(f, r) => 
            1 + len-of-list(r)
    end
where:
    list-length([list: 1, 5, 0, 8, 13]) is 5
    list-length(empty) is 0
    list-length([list: 0, 0, 0]) is 3
    list-length([list: "test", "one", " ", "two", "three", "four"]) is 6
end