
fun len<T>(l :: List<T>) -> Number:
    doc: "Compute the length of a list"
    l.length()
end




check "tests for len-of-list":
    len-of-list([list: ]) is 0
    len-of-list([list: 1, 2, 3, 4, 5, 6]) is 6
    len-of-list([list: "h", "e", "l", "l", "o"]) is 5
    len-of-list([list: "test"]) is 1
end
