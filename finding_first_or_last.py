#Returns true if given number either exist at beginning or ending.
#This is generic over any integer not just particular number
def first_last_i(i,nums):
  return nums[0]==int(i) or nums[-1]==int(i)

print(first_last_i(6,[6,34,2]))
print(first_last_i(6,[0,34,6]))
print(first_last_i(4,[5,34,6,2]))
