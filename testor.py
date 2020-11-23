#import pdb;pdb.set_trace()
# should not put when using run bazel 
#gc.set_debug(gc.DEBUG_LEAK)
import tracemalloc
tracemalloc.start(25)
#import pdb;pdb.set_trace()
#if 'dev' in sys._xoptions:
#    import gc
#    gc.enable_object_debugger(100)
#    gc.set_threshold(5)
#import gc
#gc.enable_object_debugger(100)
#gc.set_threshold(5)
import orjson

print("I have slept 10s, exit, done!")
#orjson.dumps()
#orjson.dumps([])
print("I have done!")
