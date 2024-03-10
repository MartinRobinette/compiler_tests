// Symbol table tests
#[test]
fn test_invalid_const() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public int x;
                static public char y;
                derp(int a, int b) {}
                public int[] myfunc() {}
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_double_dec_data() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public int x;
                static public char x;
                MyClass(int a, int b) {}
                public int[] myfunc() {}
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_double_dec_const() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public int x;
                static public char y;
                MyClass(int a, int b) {}
                public int[] myfunc() {}
                MyClass() {}
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_double_dec_func() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public int x;
                static public char y;
                MyClass(int a, int b) {}
                public int[] myfunc() {}
                public string myfunc() {}
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_double_dec_mix() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public int x;
                static public char y;
                MyClass(int a, int b) {}
                public int[] myfunc() {}
                public string x() {}
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_double_dec_main() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class main { 
                public int x;
                static public char y;
                main(int a, int b) {}
                public int[] myfunc() {}
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_invalid_main() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public int x;
                static public char y;
                MyClass(int a, int b) {}
                public int[] myfunc() {}
            }
            void fred(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_class_scope() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public int x;
                static public char y;
                MyClass(int a, int b) {}
                public int[] myfunc() {}
            }
            class MyClass2 { 
                public int x;
                static public char y;
                MyClass2(int a, int b) {}
                public string main() {}
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}

// Inner Symbol Table and scope tests

#[test]
fn test_scope_out() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                {
                    int y = 6;
                }
                y = 4;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_scope_class() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public int x;
                MyClass(int a, int b) {}
                public void myfunc() {
                    x = 4;
                }
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_scope_function() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public void myfunc() {
                    int x = 4;
                    x = 5;
                }
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_scope_param() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public void myfunc(int x) {
                    return x + 1;
                }
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_scope_bad() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public void myfunc(int x) {
                    return j + 1;
                }
            }
            void main(){
                int j = 4;
            }
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_double_dec_var() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int j = 4;
                char j = 'j';
            }
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_double_dec_parm() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass { 
                public void myfunc(int x) {
                    string x = "oops";
                }
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_this_in_static() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                public int x;
                static public int myfunc() {
                    return this.x + 1;
                }
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_nonstatic_in_static() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                public int x;
                static public int myfunc() {
                    return x + 1;
                }
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_static_in_static() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                static public int x;
                static public int myfunc() {
                    return x + 1;
                }
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_this_in_static_dec() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                private int x = 4;
                static private int y = this.x + 1;
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_this_in_nonstatic_dec() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                private int x = 4;
                private int y = this.x + 1;
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_instanced_dot() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                public int x;
            }
            void main(){
                MyClass c = new MyClass();
                int y = c.x;
            }
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_invalid_dot() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                public int x;
            }
            void main(){
                MyClass c = new MyClass();
                int y = c.q;
            }
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 1);
}
#[test]
fn test_nested_dot() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                public int x;
                static public MyClass myfunc() {}
            }
            void main(){
                MyClass.myfunc().x;
            }
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_instanced_static() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                static public int x;
            }
            void main(){
                MyClass c = new MyClass();
                int y = c.x;
            }
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_static_non_instanced() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                static public int x;
                static public int myfunc() {
                    return x + 1;
                }
            }
            void main(){
                MyClass.x = MyClass.myfunc();
            }
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn test_non_static_non_instanced() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                public int x;
                public int myfunc() {
                    return x + 1;
                }
            }
            void main(){
                MyClass.x = MyClass.myfunc();
            }
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 2);
}
#[test]
fn test_main_callable() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                static public void myfunc() {
                    main();
                }
            }
            void main(){}
    ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}

// Type Checking Tests

#[test]
fn dec_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 1;
                char y = 'c';
                string z = "string";
                bool b = true;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_dec_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                bool x = 1;
                int y = 'c';
                char z = "string";
                string b = true;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 4);
}
#[test]
fn assign_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x;
                char y;
                string z;
                bool b;
                x = 1;
                y = 'c';
                z = "string";
                b = true;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_assign_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x;
                char y;
                string z;
                bool b;
                b = 1;
                x = 'c';
                y = "string";
                z = true;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 4);
}
#[test]
fn binary_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 1;
                int y = x + 2;
                int z = y - x * 3;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_binary_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                char x = 'c';
                int y = x + 2;
                int z = y - x * 3;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 5);
}
#[test]
fn comp_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 1;
                char y = 'c';
                string z = "string";
                bool b = true;
                x >= 1;
                y <= 'a';
                z == "string";
                b != false;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_comp_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 1;
                char y = 'c';
                string z = "string";
                bool b = true;
                x == y;
                z <= "string";
                b >= false;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 5);
}
#[test]
fn logic_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 1;
                char y = 'c';
                x >= 1 || x <= 1;
                x == 1 && y == 'c';
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_logic_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 1;
                x >= 1 || 5;
                "apple" && x == 1;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 2);
}
#[test]
fn unary_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = -1;
                bool y = true;
                x = +x;
                !y;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_unary_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = -1;
                bool y = true;
                +y;
                !x;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 2);
}
#[test]
fn io_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x;
                char y;
                cin >> x;
                cin >> y;
                cout << 'c';
                cout << 1;
                cout << "thing";
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_io_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                string x;
                cin >> x;
                cout << true;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 2);
}
#[test]
fn loop_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 0;
                while (true) {
                    for (x=10;x>0;) {
                        if (x > 0) {
                            x = x - 1;
                        }
                    } 
                }
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_loop_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                while ('c') {
                    for (;1+1;) {
                        if (null) {}
                    } 
                }
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 3);
}
#[test]
fn newa_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {}    
            void main(){
                MyClass A = new MyClass();
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn newa_args_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                MyClass(int a, char b) {}
            }    
            void main(){
                MyClass A = new MyClass(1, 'c');
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn newa_bad_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                MyClass(int a, char b) {}
            }    
            void main(){
                MyClass A = new MyClass('c', 1);
                MyClass B = new MyClass();
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 3);
}
#[test]
fn return_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                static public int func(){
                    return 1;
                }
                static public void func2(){}
                static public void func3(){
                    return;
                }
            }    
            void main(){}
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_return_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                static public int func(){
                    return 'c';
                }
                static public int func2(){}
                static public void func3(){
                    return 1;
                }
            }    
            void main(){}
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 3);
}
#[test]
fn args_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                static public void func(int a, char b) {}
            }    
            void main(){
                MyClass.func(1, 'c');
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_args_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                static public void func(int a, char b) {}
            }    
            void main(){
                MyClass.func('c', 1);
                MyClass.func();
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 3);
}
#[test]
fn dot_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                public int x = 4;
                public void func() {}
            }    
            void main(){
                MyClass A = new MyClass();
                A.func();
                int x = A.x;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn nested_dot_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "class MyClass {
                public int x = 4;
                public void func() {}
            }
            class MyClass2 {
                public MyClass func() {
                    MyClass c = new MyClass();
                    return c;
                }
            }   
            void main(){
                MyClass2 A = new MyClass2();
                int x = A.func().x;
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn switch_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 1;
                char y = 'c';
                switch (x) {
                    case 1: x = x+1;
                    case 2: x = x+2;
                    default: x = x;
                }
                switch (y) {
                    case 'c': true;
                    case 'q': false;
                    default: false;
                }
                switch (x) {
                    default: x = 0;
                }
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn bad_switch_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int x = 1;
                switch (x) {
                    case 'c': x = x+1;
                    case 2: x = x+2;
                    default: x = x;
                }
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 2);
}
#[test]
fn newi_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int[][][] x = new int[][][10];
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}
#[test]
fn indexed_type_check() {
    let parser = kxi::CompUnitParser::new();
    let mut res = parser
        .parse(tok_gen(
            "void main(){
                int[][][] x = new int[][][10];
                x[0] = new int[][10];
                x[0][1] = new int[10];
                x[0][1][2] = 1;
                int y = x[0][1][2];
            }
        ",
        ))
        .unwrap();
    let mut st = SymbolTable::default();
    res.accept(&mut st);
    let table = st.table;
    let scope = st.c_scopes;
    let mut st = SymbolInner::new(table, scope);
    res.accept(&mut st);
    let table = st.table;
    let mut st = TypeCheck::new(table);
    res.accept(&mut st);
    assert_eq!(st.errs.len(), 0);
}

