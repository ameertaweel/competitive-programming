fun main(){
	var q = readLong()
	while(q-- > 0){
		val (n, a, b) = readLongs()
		val costPerLiterA = a
		val costPerLiterB = b.toDouble() / 2
		if(costPerLiterA <= costPerLiterB) println(costPerLiterA * n)
		else println((if(n % 2 == 0L) n * costPerLiterB else (n - 1) * costPerLiterB + costPerLiterA).toLong())
	}
}

private fun readInt() = readLine()!!.toInt()
private fun readInts() = readLine()!!.split(" ").map { it.toInt() }
private fun readLong() = readLine()!!.toLong()
private fun readLongs() = readLine()!!.split(" ").map { it.toLong() }
private fun readFloat() = readLine()!!.toFloat()
private fun readDouble() = readLine()!!.toDouble()
