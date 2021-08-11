var searchIndex = JSON.parse('{\
"jibi":{"doc":"Jibi Scheme Language Documentation","t":[13,13,13,13,13,13,13,13,13,13,13,13,13,3,3,6,3,4,6,3,6,3,4,6,13,13,13,13,13,17,13,3,3,13,13,13,13,13,13,13,13,13,3,3,8,3,4,3,3,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,12,11,11,12,12,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11],"n":["Anychar","ApplyError","AssertionError","Bool","Builtin","Env","Eof","Error","EvalError","Exception","Ident","Int","Int","Interpreter","JEnv","JEnvRef","JError","JErrorKind","JException","JPair","JResult","JState","JVal","JValRef","LParen","Lambda","Nil","NotDefined","OsError","PRELUDE","Pair","Parser","PositionTag","ProcMacro","Quote","Quote","RParen","SpecialForm","String","String","Symbol","SyntaxError","Token","TokenError","TokenIter","TokenValidator","TokenValue","Tokenizer","TracebackFrame","TypeError","UserDefined","add_reader_macro","bool","bool","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","builtin","builtin","call","car","cdr","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","col","cons","def","default","default","default","default","define","eq","eq","eq","eq","eq","eq","eq","error","error","eval","eval_file","eval_file","eval_str","eval_str","eval_tokens","eval_tokens","filename","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from_any","from_builtin","from_lambda","import_module","input","int","int","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into_ref","into_ref","is_list","is_list","is_same_kind","iter","iter_list","kind","lambda","lambda","lineno","list","list","lookup","lookup","ne","ne","ne","ne","ne","ne","new","new","new","new","new","new","new","new","new","new","next_token","next_token","nil","nil","pair","pair","parent","parse_form","parse_forms","pos","pos","print_exc","procmacro","procmacro","quote","quote","reason","reason","set","specialform","specialform","string","string","symbol","symbol","to_bool","to_env","to_error","to_int","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_pair","to_str","to_string","to_string","to_string","to_string","to_string","to_string","to_string","to_symbol","tokenize","traceback","traceback_push","traceback_take","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_lookup","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","value","with_lineno"],"q":["jibi","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Construct a <code>jibi</code> <code>bool</code> (always interned).","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Define a <code>jibi</code> builtin procedure.","","Call a named <code>jibi</code> procedure.","","","","","","","","","","","","","","","","","","","","","Create a global binding (variable definition).","Create <code>jibi</code> interpreter pre-loaded with builtins and …","","","","Create a new binding.","","","","","","","","Construct a <code>jibi</code> <code>error</code> value.","","Evaluate a <code>jibi</code> expression. (All values are expressions, …","Evaluate a <code>jibi</code> script file, and return the value of the …","","Evaluate a <code>jibi</code> script, and return the value of its last …","","Evaluate a stream of tokens and return the value of the …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns None when more input is expected based on …","","","","","","","","","","","","","","","","","","","","","","","","","","Construct a <code>jibi</code> <code>lambda</code>.","","","Construct a <code>jibi</code> list (linked list made from <code>pair</code> and …","","Get value of binding in global environment.","","","","","","","","Create a bare <code>jibi</code> [<code>Interpreter</code>], with no builtins or …","","","","","","","","","","","","Construct a <code>jibi</code> <code>nil</code> (always interned).","","Construct a <code>jibi</code> <code>pair</code> (cons cell).","","","","","","","Print exception and traceback.","Construct a <code>jibi</code> <code>procmacro</code>.","","Construct a <code>jibi</code> <code>quote</code>.","","","","Change existing binding.","Define a <code>jibi</code> builtin special form.","","Construct a <code>jibi</code> <code>string</code> (may be interned).","","Construct a <code>jibi</code> <code>symbol</code> (always interned).","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,2,2,3,3,3,1,3,2,2,1,1,3,0,0,0,0,0,0,0,0,0,0,0,1,3,3,2,2,0,3,0,0,3,1,3,1,3,1,3,3,2,0,0,0,0,0,0,0,2,2,4,5,4,5,6,7,4,8,2,9,1,10,11,12,13,14,15,3,5,6,7,4,8,2,9,1,10,11,12,13,14,15,3,5,4,5,15,15,8,2,9,1,13,14,15,3,8,2,9,1,13,14,15,3,13,15,5,5,4,8,11,8,8,2,9,1,13,15,3,5,4,5,5,4,5,4,5,4,13,8,8,2,2,9,9,1,10,12,12,13,13,14,14,15,3,3,5,6,7,4,8,2,9,1,10,11,12,13,14,15,3,14,14,14,4,11,5,4,5,6,7,4,8,2,9,1,10,11,12,13,14,15,3,8,3,15,3,9,15,3,9,5,4,13,5,4,5,8,2,9,1,13,15,3,5,6,7,4,8,9,10,11,12,13,16,7,5,4,5,4,8,6,6,10,12,5,5,4,5,4,9,12,8,5,4,5,4,5,4,3,3,3,3,8,2,9,1,13,14,15,3,3,3,8,2,9,12,13,14,3,3,7,4,4,4,5,6,7,4,8,2,9,1,10,11,12,13,14,15,3,5,6,7,4,8,2,9,1,10,11,12,13,14,15,3,8,5,6,7,4,8,2,9,1,10,11,12,13,14,15,3,10,7],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["readermacro",3]]],[[["bool",15]],["jvalref",6]],[[["bool",15]],["jvalref",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["string",3]],["jvalref",6]],[[["rc",3],["string",3],["fn",8]],["jvalref",6]],[[["jvalref",6],["str",15],["vec",3]],["jresult",6]],[[],["jvalref",6]],[[],["jvalref",6]],[[],["jenv",3]],[[],["jerrorkind",4]],[[],["jerror",3]],[[],["tokenvalue",4]],[[],["positiontag",3]],[[],["tracebackframe",3]],[[],["jpair",3]],[[],["jval",4]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["jvalref",6]]],[[["jvalref",6],["str",15]]],[[]],[[]],[[]],[[],["tokenvalidator",3]],[[["jvalref",6],["str",15]]],[[["jenv",3]],["bool",15]],[[["jerrorkind",4]],["bool",15]],[[["jerror",3]],["bool",15]],[[["tokenvalue",4]],["bool",15]],[[["positiontag",3]],["bool",15]],[[["jpair",3]],["bool",15]],[[["jval",4]],["bool",15]],[[["str",15],["jerrorkind",4]],["jvalref",6]],[[["str",15],["jerrorkind",4]],["jvalref",6]],[[["jvalref",6]],["jresult",6]],[[["asref",8],["path",3]],[["option",4],["result",4],["jexception",6]]],[[["jenvref",6],["asref",8],["path",3]],[["result",4],["option",4]]],[[["str",15]],[["option",4],["result",4],["jexception",6]]],[[["jenvref",6],["str",15]],[["result",4],["option",4]]],[[["box",3],["tokeniter",8]],[["option",4],["result",4],["jexception",6]]],[[["box",3],["jenvref",6],["tokeniter",8]],[["result",4],["option",4]]],null,[[["formatter",3]],[["result",4],["error",3]]],[[["formatter",3]],[["result",4],["error",3]]],[[["formatter",3]],[["result",4],["error",3]]],[[["formatter",3]],["result",6]],[[["formatter",3]],[["result",4],["error",3]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],[["result",4],["error",3]]],[[["formatter",3]],["result",6]],[[["formatter",3]],[["result",4],["error",3]]],[[["formatter",3]],["result",6]],[[["formatter",3]],[["result",4],["error",3]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],[["result",4],["error",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["jenvref",6],["jvalref",6]],["option",4]],[[["jenvref",6],["jvalref",6]],["option",4]],[[["jvalref",6]],["option",4]],[[["jenvref",6],["asref",8],["path",3]],["jresult",6]],[[["string",3]],[["option",4],["result",4],["tokenerror",3]]],[[["i128",15]],["jvalref",6]],[[["i128",15]],["jvalref",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["jenvref",6]],[[],["jvalref",6]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],[["result",4],["jlistiterator",3],["jerror",3]]],[[],[["result",4],["jlistiterator",3],["jerror",3]]],null,[[["vec",3],["vec",3],["string",3],["jvalref",6]],["jresult",6]],[[["jenvref",6],["vec",3],["vec",3],["string",3],["jvalref",6]],["jresult",6]],null,[[["jvalref",6],["vec",3]],["jvalref",6]],[[["jvalref",6],["vec",3]],["jvalref",6]],[[["str",15]],[["option",4],["jvalref",6]]],[[["str",15]],[["option",4],["jvalref",6]]],[[["jerrorkind",4]],["bool",15]],[[["jerror",3]],["bool",15]],[[["tokenvalue",4]],["bool",15]],[[["positiontag",3]],["bool",15]],[[["jpair",3]],["bool",15]],[[["jval",4]],["bool",15]],[[]],[[["box",3],["tokeniter",8],["jstate",3]]],[[["string",3]]],[[]],[[["jenvref",6],["option",4]]],[[["str",15],["jerrorkind",4]]],[[["tokenvalue",4],["positiontag",3]]],[[["str",15]]],[[["str",15],["positiontag",3]]],[[["usize",15],["str",15]]],[[],[["result",4],["tokenerror",3],["token",3]]],[[],[["result",4],["tokenerror",3],["token",3]]],[[],["jvalref",6]],[[],["jvalref",6]],[[["jvalref",6]],["jvalref",6]],[[["jvalref",6]],["jvalref",6]],null,[[],[["result",4],["option",4],["parsererror",3]]],[[],[["parsererror",3],["result",4],["vec",3]]],null,null,[[["jexception",6]]],[[["vec",3],["vec",3],["string",3],["jvalref",6]],["jresult",6]],[[["jenvref",6],["vec",3],["vec",3],["string",3],["jvalref",6]],["jresult",6]],[[["jvalref",6]],["jvalref",6]],[[["jvalref",6]],["jvalref",6]],null,null,[[["jstate",3],["jvalref",6],["str",15]],[["result",4],["jerror",3]]],[[["string",3]],["jvalref",6]],[[["rc",3],["string",3],["fn",8]],["jvalref",6]],[[["string",3]],["jvalref",6]],[[["string",3]],["jvalref",6]],[[["string",3]],["jvalref",6]],[[["string",3]],["jvalref",6]],[[],[["bool",15],["result",4],["jerror",3]]],[[],[["jenvref",6],["result",4],["jerror",3]]],[[],[["jerror",3],["result",4],["jerror",3]]],[[],[["jerror",3],["result",4],["i128",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],[["result",4],["jpair",3],["jerror",3]]],[[],[["jerror",3],["str",15],["result",4]]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],[["jerror",3],["str",15],["result",4]]],[[],[["vec",3],["tokenerror",3],["result",4]]],[[]],[[["option",4],["tracebackframe",3]]],[[],[["vec",3],["tracebackframe",3]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["str",15]],["jresult",6]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,[[["string",3],["usize",15]]]],"p":[[4,"TokenValue"],[4,"JErrorKind"],[4,"JVal"],[3,"JState"],[3,"Interpreter"],[3,"Parser"],[3,"Tokenizer"],[3,"JEnv"],[3,"JError"],[3,"Token"],[3,"TokenValidator"],[3,"TokenError"],[3,"PositionTag"],[3,"TracebackFrame"],[3,"JPair"],[8,"TokenIter"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};