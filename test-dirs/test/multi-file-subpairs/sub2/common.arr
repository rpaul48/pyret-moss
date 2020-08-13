provide *

data CustomList<A>:
	# here is the empty case: this happens if there are no elements in the list
	| custom-empty

	# here is the link case: if there is 1 or more elements in the CustomList
	| custom-link(cust-first :: A, cust-rest :: CustomList<A>)
end
