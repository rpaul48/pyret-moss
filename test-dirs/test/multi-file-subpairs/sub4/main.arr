provide *

# this is my function below:
fun compute-the-square(num) -> Number:
	#| it computes the square of num |#
	num * num
end

check:
	compute-the-square(5) is 25
	compute-the-square(7) is 49
	compute-the-square(-1) is 1
	compute-the-square(2) is 4
end
