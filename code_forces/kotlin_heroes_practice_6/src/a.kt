fun main(){
	readInt() // We don't need to use n in our code
	val a = readInts()
	val solution = a.reversed().toSet().reversed()
	println(solution.size)
	println(solution.joinToString(" "))
}

private fun readInt() = readLine()!!.toInt()

private fun readInts() = readLine()!!.split(" ").map { it.toInt() }
