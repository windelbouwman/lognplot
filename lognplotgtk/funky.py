""" Read in funky hdf5 file.

"""

import h5py

f = h5py.File('datorz.h5', 'r')
print(f)

print(f.keys())

group = f['my_datorz']
print(group.keys())
signal_names = group.keys()
for name in signal_names:
    print('Signal ', name)
    ds = group[name]
    print(ds)
    print(ds[:])

