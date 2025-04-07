# LeetCode/252 - Meeting Rooms


class Interval:
    def __init__(self, start: int, end: int):
        self.start: int = start
        self.end: int = end


class Solution:
    def canAttendMeetings(self, intervals: list[Interval]) -> bool:
        if len(intervals) == 0:
            return True
        intervals.sort(key=lambda x: x.start)
        last_end = intervals[0].start
        for i in range(len(intervals)):
            if intervals[i].start < last_end:
                return False
            last_end = intervals[i].end
        return True
