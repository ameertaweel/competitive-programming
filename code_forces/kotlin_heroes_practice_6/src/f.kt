fun main(){
	val (n, m, k) = readInts()
	val a = readInts().toMutableList()
	val maxSum = m * k
	var sum = a.sum()
	while(sum > maxSum){
		sum -= a[0]
		a.removeFirst()
	}
	while(true){
		var usedBoxes = 1
		var usedSizeOfCurrentBox = 0
		for(i in 0 until a.size){
			if(usedSizeOfCurrentBox + a[i] <= k) usedSizeOfCurrentBox += a[i]
			else {
				usedBoxes += 1
				usedSizeOfCurrentBox = a[i]
			}
			if(usedBoxes > m) break
		}
		if(usedBoxes > m) a.removeFirst()
		else break
	}
	println(a.size)
}

private fun readInt() = readLine()!!.toInt()
private fun readInts() = readLine()!!.split(" ").map { it.toInt() }
private fun readLong() = readLine()!!.toLong()
private fun readLongs() = readLine()!!.split(" ").map { it.toLong() }
private fun readFloat() = readLine()!!.toFloat()
private fun readDouble() = readLine()!!.toDouble()
private fun readString() = readLine()!!
private fun readStrings() = readLine()!!.split(" ")
