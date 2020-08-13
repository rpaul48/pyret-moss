provide *

fun testing(x :: Number) -> Number:
	doc: "This function computes the square of the input"
	x * x
where:
	testing(5) is 25
	testing(7) is 49
	testing(-1) is 1
end

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

batting-avg-and-slugging = extend batting
  using at-bats, singles, doubles, triples, home-runs:
  batting-average: (singles + doubles + triples + home-runs) / at-bats,
  slugging-percentage: (singles + (doubles * 2) +
    (triples * 3) + (home-runs * 4)) / at-bats
end
