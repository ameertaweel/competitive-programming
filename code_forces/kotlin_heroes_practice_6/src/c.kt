fun main(){
	readLong() // We do not need to use n in our cod3
	val name = readString()
	var xInARow = 0
	var charsToRemove = 0
	name.forEach {
		if(it == 'x') xInARow++
		else xInARow = 0

		if(xInARow > 2) charsToRemove++
	}
	println(charsToRemove)
}

private fun readInt() = readLine()!!.toInt()
private fun readInts() = readLine()!!.split(" ").map { it.toInt() }
private fun readLong() = readLine()!!.toLong()
private fun readLongs() = readLine()!!.split(" ").map { it.toLong() }
private fun readFloat() = readLine()!!.toFloat()
private fun readDouble() = readLine()!!.toDouble()
private fun readString() = readLine()!!
