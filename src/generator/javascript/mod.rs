extern crate regex;
use self::regex::Regex;


use std::fs::*;
use std::io::*;

use ast::*;
use ast::ASTAction::*;

use transformer::javascript::js_ast::*;
use transformer::javascript::js_ast::JSASTField::*;
use transformer::javascript::js_ast::JSASTItemType::*;
use transformer::javascript::js_ast::JSASTGroupType::*;
use transformer::javascript::js_ast::JSASTLiteral::*;

mod test;


pub type Code = String;


trait Generator {
    /// Generate code without indentation
    fn generate(&self) -> Code {
        self.generate_ident("".to_string())
    }


    /// Generate code with indentation
    fn generate_ident(&self, ident: String) -> Code {
        ident + &self.generate()
    }
}


impl Generator for JSASTLiteral {
    fn generate(&self) -> Code {
        match *self {
            JSString(ref x) => format!("\"{}\"", x),
            JSNumber(x) => format!("{}", x),
            JSName(ref x) => format!("{}", x),
            JSComment(ref x) => format!("/* {} */", x),
            JSUndefined() => "undefined".to_string(),
        }
    }


    // Literals don't need ident, unless they are comments
    fn generate_ident(&self, ident: String) -> Code {
        match *self {
            JSComment(_) => ident + &self.generate(),
            _ => self.generate(),
        }
    }
}


macro_rules! variable_scope {
    ($name: expr) => {{
        if $name.chars().nth(0).unwrap_or('+') == '_' {
            "this".to_string() + $name
        }
        else {
            $name.to_string()
        }
    }}
}


impl Generator for ASTAction {
    fn generate_ident(&self, ident: String) -> Code {
        match *self {
            PopValueFromStack() => ident + "$stack.pop();",

            Return() => ident + "return;",

            SwapTwoFromStack() =>
                ident.clone() + "var y = $stack.pop();\n"
                + &ident.clone() + "var x = $stack.pop();\n"
                + &ident.clone() + "$stack.push(y);\n"
                + &ident.clone() + "$stack.push(x);",

            PopAndRunFunction() => ident + "$stack.pop()();",

            Assign(ref name, ref value) =>
                ident + &format!(
                    "{name} = {value};",
                    name = variable_scope!( &JSASTLiteral::from(name.clone()).generate() ),
                    value = JSASTLiteral::from(value.clone()).generate(),
                ),

            AssignFromStack() =>
                // Shortcut because it's so common
                ident.clone() + "$assignFromStack(this);",

            NewInstance(ref name, ref cname) =>
                ident + &format!("this.{} = new {}();",
                    JSASTLiteral::from(name.clone()).generate(),
                    JSASTLiteral::from(cname.clone()).generate(),
                ),

            NewInstanceFromStack() =>
                ident.clone() + "var cname = $stack.pop();\n"
                + &ident.clone() + "var name = $stack.pop();\n"
                + &ident.clone() + "this[name] = new this[cname]();",

            RetrieveFunction(ref oname, ref fname) =>
                // Using bind, because we would otherwise lose the prototypes
                ident + &format!("$stack.push(this.{}.{}.bind(this.{0}));",
                    JSASTLiteral::from(oname.clone()).generate(),
                    JSASTLiteral::from(fname.clone()).generate(),
                ),

            RetrieveFunctionFromStack() =>
                ident.clone() + "var fname = $stack.pop();\n"
                + &ident.clone() + "var oname = $stack.pop();\n"
                + &ident.clone() + "$stack.push(this[oname][fname].bind(this[oname]));",

            RetrieveFromName(ref name) =>
                ident + &format!("$stack.push(this.{});",
                    JSASTLiteral::from(name.clone()).generate(),
                ),

            RetrieveFromNameFromStack() =>
                ident.clone() + "var name = $stack.pop();\n"
                + &ident.clone() + "$stack.push(this[name]);",

            AssignCurrent(ref name) =>
                ident + &format!("this.{} = this;",
                    JSASTLiteral::from(name.clone()).generate(),
                ),

            AssignCurrentFromStack() =>
                ident.clone() + "var name = $stack.pop()\n"
                + &ident.clone() + "this[name] = this;",



            PushLiteralToStack(ref x) =>
                ident + &format!("$stack.push({});",
                    // When pushing a name to the stack, it should be a string,
                    // so that `this[name]` can be called
                    match *x {
                        ASTLiteral::Name(ref y) => format!("\"{}\"", y),
                        _ => JSASTLiteral::from(x.clone()).generate(),
                    }
                ),
        }
    }
}


impl Generator for JSASTItemType {
    fn generate_ident(&self, ident: String) -> Code {
        match *self {
            JSAction(ref x) => x.generate_ident(ident),
            JSLiteral(ref x) => x.generate_ident(ident),
        }
    }
}


impl Generator for JSASTField {
    fn generate_ident(&self, ident: String) -> Code {
        match *self {
            JSASTItem(ref x) => x.generate_ident(ident),

            JSASTGroup(JSBlock(), ref body) =>
                ident.clone() + &format!("{{\n{}{}}}\n",
                    body.generate_ident(ident.clone() + "\t"),
                    ident.clone(),
                ),

            JSASTGroup(JSFunction(ref fname), ref body) =>
                ident.clone() + &format!("{fname} = function() {{\n{body}{ident}}}\n",
                    fname = fname.generate(),
                    body = body.generate_ident(ident.clone() + "\t"),
                    ident = ident.clone(),
                ),

            JSASTGroup(JSWhile(ref field), ref body) =>
                ident.clone() + &format!("while (this.{}) {{\n{}{}}}\n",
                    field.generate(),
                    body.generate_ident(ident.clone() + "\t"),
                    ident.clone(),
                ),
        }
    }
}


impl Generator for JSAST {
    fn generate_ident(&self, ident: String) -> Code {
        let mut code = String::new();

        for field in self {
            code += & (*field).generate_ident(ident.clone());
            code += "\n";
        }

        code
    }
}


pub fn optimize(mut code: Code) -> Code {
    // HACK: Replace $stack.push(x);\n$stack.pop()();  with x();
    let rx = Regex::new(r"(?mx)
        ^(?P<w>\W*)  # ident

        \$stack\.push\(
            (?P<a>[^)]+)\.bind\([^)]+\)
        \);\n

        \W* \$stack\.pop\(\)\(\);
    ").unwrap();

    code = rx.replace_all(&code, "${w}$a();").to_string();


    // HACK: Replace $stack.push(x);\ny = $stack.pop();  with  y = x
    let rx = Regex::new(r"(?mx)
        ^(?P<w>\W*)  # ident

        \$stack\.push\(
            (?P<a>[^)]+)
        \);\n

        \W* (?P<b>\w+) = \$stack\.pop\(\);
    ").unwrap();

    code = rx.replace_all(&code, "${w}$b = $a;").to_string();


    // HACK: Replace $stack.push(x);\n$stack.push(y);  with  $stack.push(x, y);
    let rx = Regex::new(r"(?mx)
        ^(?P<w>\W*)

        \$stack\.push\(
            (?P<a>[^)]+)
        \);\n

        \W* \$stack\.push\(
            (?P<b>[^)]+)
        \);
    ").unwrap();

    // HACK: 3 times to make sure we got everything (up to 8 items)
    code = rx.replace_all(&code, "${w}$$stack.push($a, $b);").to_string();
    code = rx.replace_all(&code, "${w}$$stack.push($a, $b);").to_string();
    code = rx.replace_all(&code, "${w}$$stack.push($a, $b);").to_string();


    // HACK: Replace $stack.push(x, y);\n$assignFromStack(this);  with  this[x] = y;
    let rx = Regex::new(r"(?mx)
        ^(?P<w>\W*)  # ident

        \$stack\.push\(
            (?P<a>[^),]+),\s
            (?P<b>[^),]+)
        \);\n

        \W*\$assignFromStack\(this\);
    ").unwrap();

    code = rx.replace_all(&code, "${w}this[$a] = $b;").to_string();


    // HACK: Replace while (x) { ...; return; }  with  if (x) { ... }
    let rx = Regex::new(r"(?mx)
        ^(?P<w>\W*)  # ident

        while\s \( (?P<s>\w+) \)\s \{\n
            (?P<b>(\W*\w+\W*)*\n)
            \W*return;\n
        \W*\}
    ").unwrap();

    code = rx.replace_all(&code, "${w}if ($s) {\n${b}${w}\treturn;\n${w}}").to_string();


    // HACK: Replace this\["(\w+)"\] with $1
    let rx = Regex::new(r#"this\["(\w+)"\]"#).unwrap();
    code = rx.replace_all(&code, "$1").to_string();


    // HACK: Replace this.this with this
    /*
    let rx = Regex::new(r"this\.this").unwrap();
    code = rx.replace_all(&code, "this").to_string();
    */


    // HACK: Remove this = this
    let rx = Regex::new(r"(?m)^\W*this = this;$\n").unwrap();
    code = rx.replace_all(&code, "").to_string();


    code
}


#[cfg_attr(feature="clippy", allow(ptr_arg))]
/// Optimization is unstable; set `to_optimize` to false when testing
pub fn generate_without_library(jsast: &JSAST, to_optimize: bool) -> Code {
    if to_optimize {
        optimize(jsast.generate())
    }
    else {
        jsast.generate()
    }
}


#[cfg_attr(feature="clippy", allow(ptr_arg))]
pub fn generate_with_library(jsast: &JSAST) -> Code {
    let mut code = String::new();

    // Add library
    code += "// BEGIN LIBRARY \n\n";

    let mut lib_file = File::open("src/generator/javascript/lib.min.js")
        .expect("Can't find minified library file `lib.min.js` in src folder");
    lib_file.read_to_string(&mut code).expect("Failed to read library file");

    code += "\n// END LIBRARY \n\n";

    // Add actual code
    code += &generate_without_library(jsast, true);
    code += "\n";

    // Add main function
    code += "if(typeof M !== 'undefined' && M.prototype.m) new M().m();\n";

    code
}
