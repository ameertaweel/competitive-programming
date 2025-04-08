# LeetCode/253 - Meeting Rooms II


class Interval(object):
    def __init__(self, start: int, end: int):
        self.start: int = start
        self.end: int = end


class Solution:
    def minMeetingRooms(self, intervals: list[Interval]) -> int:
        time: list[tuple[int, int]] = []
        for i in intervals:
            time.append((i.start, 1))
            time.append((i.end, -1))

        # Sorts by the first tuple element then by the second.
        # This order is important so that a meeting that ends at time t
        # does not conflict with a meeting that starts at time t.
        time.sort()

        days = 0
        count = 0

        for t in time:
            count += t[1]
            days = max(days, count)

        return days
