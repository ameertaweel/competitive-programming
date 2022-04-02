class Solution {
	public void reverseString(char[] s) {
		for (int i = 0, j = s.length - 1; ; i++, j--){
			if (i >= j) break;
			char tmp = s[i];
			s[i] = s[j];
			s[j] = tmp;
		}
		System.out.println(s);
	}
}
