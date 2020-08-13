fun add-all(l :: List<Number>) -> Number:
    doc: "adds all the numbers in the input list"
    cases (List) l:
        | empty => 0
        | link(f, rest) => f + add-all(rest)
    end
end

check "tests for add-all":
    len-of-list([list: 1, 5, 0, 8, 13]) is 27
    len-of-list(empty) is 0
    len-of-list([list: 0, 0, 0]) is 0
    len-of-list([list: 2.4, 2, 0]) is 4.4
end


check "":
  y = for fold(sum from 0, elem from [list: 1,2,3]):
    sum + elem
  end
  y is 6
end
