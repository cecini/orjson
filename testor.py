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
#jsonb = to_json_bytes(input)

jsonb = orjson.dumps({"kind": "beginb"})
jsonc = orjson.dumps(dict(kind="begin"))

print(jsonb)
print(jsonc)
#print(orjson.dumps(dict(kind="begin")))
#print(orjson.dumps(dict(kind="begin")))
# print(orjson.dumps(dict(kind="begin")))

data = {
    "type": "job",
    #"created_at": datetime.datetime(1970, 1, 1),
    "created_at": 1,
    "status": "🆗",
    #"payload": numpy.array([[1, 2], [3, 4]]),
    "payload": 222,
}
print(orjson.dumps(data))
import decimal
def default(obj):
    if isinstance(obj, decimal.Decimal):
        return str(obj)

print(orjson.dumps({"set":{1, 2}}, default=default))
print(orjson.dumps({"a": "b", "c": {"d": True}, "e": [1, 2]}))
#b'{"a":"b","c":{"d":true},"e":[1,2]}'

print("I have done!")
# even have valgrindi with release build
#valgrind+ python debug _ gc_disable ,but cannot trace extension module when python finaliztion"
# valgrind _ python release, so use ndebug ,now no pyhont finaliztion faild, but the extension module leak cannot find. why

