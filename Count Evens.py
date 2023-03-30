#Counting the even numbers in the collection not summing the even numbers unless you set the sum is True.
def count_evens(Iterable_type,sum=False):
  even_count=0
  even_sum = 0
  for i in Iterable_type:
    if i % 2 == 0:
      even_count+=1
      even_sum +=i

  if sum:
    return "Even counts is {0},Sum of even number is {1}".format(even_count,even_sum)
  return even_count
  
print(count_evens([2,4,8,16,32,64,7,9]))
print(count_evens([1,3,5,7,9]))
print(count_evens([2,4,3,5,8],sum=True))
