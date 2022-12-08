#!/usr/bin/env python

l = open("input.txt").readlines()

d = {}
pwd = "/"

for p in l:
  v = p.split()
  if v[0] == "$":
    if v[1] == "cd":
      if v[2][0] == "/":
        pwd = v[2]
      elif v[2] == "..":
        pwd = pwd[:pwd.rfind("/")]
      else:
        if pwd != "/":
            pwd = pwd+"/"+v[2]
        else:
            pwd = pwd+v[2]
  else:
    if v[0] != "dir":
      dest = pwd+"/"+v[1]
      if pwd == "/":
        dest = "/"+v[1] 
      d[dest]  = int(v[0])

dirsizes = {"/": 0}

for p in d:
  dir = p[:p.rfind("/")]

  while len(dir) > 0:
    if dir in dirsizes:
        dirsizes[dir] = dirsizes[dir] + d[p]
    else:
        dirsizes[dir] = d[p]
    dir = dir[:dir.rfind("/")]
  dirsizes["/"] = dirsizes["/"]+d[p]

count  = 0
for p in dirsizes:
   if dirsizes[p] < 100000:
    count = count+dirsizes[p]
print(count)
