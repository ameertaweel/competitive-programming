fun main(){
	val n = readInt()

	val strings = mutableListOf<String>()
	for(i in 1..n) strings.add(readString())

	strings.sortBy { it.length }

	for(i in n - 1 downTo 1){
		val parent = strings[i]
		val child = strings[i - 1]
		if(child !in parent){
			println("NO")
			return
		}
	}

	println("YES")
	print(strings.joinToString("\n"))
}

private fun readInt() = readLine()!!.toInt()
private fun readInts() = readLine()!!.split(" ").map { it.toInt() }
private fun readLong() = readLine()!!.toLong()
private fun readLongs() = readLine()!!.split(" ").map { it.toLong() }
private fun readFloat() = readLine()!!.toFloat()
private fun readDouble() = readLine()!!.toDouble()
private fun readString() = readLine()!!
