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
# if add import gc ;gc.set_threshold(5) erro is enum and malloc stack long 

import gc
#gc.set_debug(gc.DEBUG_LEAK|gc.DEBUG_STATS)
gc.disable()
#gc.get_objects()

# need gc 
#from pympler import tracker
#tr = tracker.SummaryTracker()
#import orjson
#tr.print_diff()
#import uuid
import orjson
#gc.get_objects()



#  gc.get_stats()

# print("I have slept 10s, exit, done!")
#orjson.dumps()
#orjson.dumps([])
#  print("I have done!")
# even have valgrindi with release build
#valgrind+ python debug _ gc_disable ,but cannot trace extension module when python finaliztion"
# valgrind _ python release, so use ndebug ,now no pyhont finaliztion faild, but the extension module leak cannot find. why

