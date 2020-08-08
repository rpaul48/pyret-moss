# Shared list definition that everyone gets as boilerplate
data MyList<T>:
	| my-empty
	| my-link(first :: T, rest :: List<T>)
end