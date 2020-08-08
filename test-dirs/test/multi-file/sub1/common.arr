provide *
provide-types *

# Here's a data definition:
data MyList<T>:
	| my-empty
	| my-link(first :: T, rest :: List<T>)
end