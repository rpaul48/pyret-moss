provide *

data CustomList<A>:
	# here is the empty case: this happens if there are no elements in the list
	| custom-empty

	# here is the link case: if there is 1 or more elements in the CustomList
	| custom-link(cust-first :: A, cust-rest :: CustomList<A>)
end

# some random code

fun replicate(n :: Number%(non-negative), e) -> List:
  if n == 0:
    [list:]
  else:
    link(e, replicate(n - 1, e))
  end
end

check:
  n = for map(elem from [list: 1,2,3,4]):
    elem + 2
  end
  n is [list: 3,4,5,6]
end

check:
  z = for filter(elem from [list: 1,2,3,4]):
    elem < 3
  end
  z is [list: 1,2]
end

check:
  y = for fold(sum from 0, elem from [list: 1,2,3]):
    sum + elem
  end
  y is 6
end

m = 100
if m < 10:
  print("Small")
else if m > 20:
  print("Large")
else:
  print("Medium")
end

data CustomList<A>:
	# here is the empty case: this happens if there are no elements in the list
	| custom-empty

	# here is the link case: if there is 1 or more elements in the CustomList
	| custom-link(cust-first :: A, cust-rest :: CustomList<A>)
end

# some random code

fun replicate(n :: Number%(non-negative), e) -> List:
  if n == 0:
    [list:]
  else:
    link(e, replicate(n - 1, e))
  end
end

check:
  n = for map(elem from [list: 1,2,3,4]):
    elem + 2
  end
  n is [list: 3,4,5,6]
end

check:
  z = for filter(elem from [list: 1,2,3,4]):
    elem < 3
  end
  z is [list: 1,2]
end

check:
  y = for fold(sum from 0, elem from [list: 1,2,3]):
    sum + elem
  end
  y is 6
end

m = 100
if m < 10:
  print("Small")
else if m > 20:
  print("Large")
else:
  print("Medium")
end

data CustomList<A>:
	# here is the empty case: this happens if there are no elements in the list
	| custom-empty

	# here is the link case: if there is 1 or more elements in the CustomList
	| custom-link(cust-first :: A, cust-rest :: CustomList<A>)
end

# some random code

fun replicate(n :: Number%(non-negative), e) -> List:
  if n == 0:
    [list:]
  else:
    link(e, replicate(n - 1, e))
  end
end

check:
  n = for map(elem from [list: 1,2,3,4]):
    elem + 2
  end
  n is [list: 3,4,5,6]
end

check:
  z = for filter(elem from [list: 1,2,3,4]):
    elem < 3
  end
  z is [list: 1,2]
end

check:
  y = for fold(sum from 0, elem from [list: 1,2,3]):
    sum + elem
  end
  y is 6
end

m = 100
if m < 10:
  print("Small")
else if m > 20:
  print("Large")
else:
  print("Medium")
end

data CustomList<A>:
	# here is the empty case: this happens if there are no elements in the list
	| custom-empty

	# here is the link case: if there is 1 or more elements in the CustomList
	| custom-link(cust-first :: A, cust-rest :: CustomList<A>)
end

# some random code

fun replicate(n :: Number%(non-negative), e) -> List:
  if n == 0:
    [list:]
  else:
    link(e, replicate(n - 1, e))
  end
end

check:
  n = for map(elem from [list: 1,2,3,4]):
    elem + 2
  end
  n is [list: 3,4,5,6]
end

check:
  z = for filter(elem from [list: 1,2,3,4]):
    elem < 3
  end
  z is [list: 1,2]
end

check:
  y = for fold(sum from 0, elem from [list: 1,2,3]):
    sum + elem
  end
  y is 6
end

m = 100
if m < 10:
  print("Small")
else if m > 20:
  print("Large")
else:
  print("Medium")
end
