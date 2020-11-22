#import pdb;pdb.set_trace()
# should not put when using run bazel 
import gc 
#gc.set_debug(gc.DEBUG_LEAK)
gc.disable()
import tracemalloc
tracemalloc.start(25)
gc.enable()
import orjson
gc.disable()

import time
# PYTHONMALLOCSTATS=1
# PYTHONTRACEMALLOC=25
print("I have loaeded the orjson library")

time.sleep(2)
print("I have slept 10s, exit, done!")
#gc.set_debug(gc.DEBUG_LEAK)
#to_json_bytes = orjson.dumps
orjson.dumps([])

print("I have done!")
