import kotlin.math.min

fun main(){
	val q = readInt()

	val shelf = mutableMapOf<Long, Long>()
	var minPosition = 0L
	var maxPosition = 1L
	for(i in 1..q){
		val (queryType, id) = readStrings()
		when(queryType) {
			"L" -> shelf[id.toLong()] = minPosition--
			"R" -> shelf[id.toLong()] = maxPosition++
			else -> {
				val index = shelf[id.toLong()]!!
				val booksFromLeft = index - minPosition - 1
				val booksFromRight = maxPosition - index - 1
				println(min(booksFromLeft, booksFromRight))
			}
		}
	}
}

private fun readInt() = readLine()!!.toInt()
private fun readInts() = readLine()!!.split(" ").map { it.toInt() }
private fun readLong() = readLine()!!.toLong()
private fun readLongs() = readLine()!!.split(" ").map { it.toLong() }
private fun readFloat() = readLine()!!.toFloat()
private fun readDouble() = readLine()!!.toDouble()
private fun readString() = readLine()!!
private fun readStrings() = readLine()!!.split(" ")
