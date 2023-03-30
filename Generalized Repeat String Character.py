#Generic over the repeat count instead of specific to single implementation
def multiple_char(st,repeat):
  output=str("")
  for char in str(st):
    output+=str(char)*int(repeat)
  return output

print(multiple_char("Hi-There",2))
print(multiple_char("Hi-There",3))
print(multiple_char("Hi-There",4))
