provide *

fun testing(x :: Number) -> Number:
	doc: "This function computes the square of the input"
	x * x
where:
	testing(5) is 25
	testing(7) is 49
	testing(-1) is 1
end

fun apply-twice(f, x):
  doc: "Here's my docstring"
  f(f(x))
where:
  apply-twice(sqa, 2) is 16
  apply-twice(sqa, 3) is 81
end
