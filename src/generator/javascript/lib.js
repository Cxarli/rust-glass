//jshint node: true, -W014
'use strict';'REMOVE AFTER MINIFYING';

// Node has support for no-newline
var $output = (typeof process !== 'undefined')
  ? process.stdout.write.bind(process.stdout)
  : console.log;

var $stack = [];


function $assignFromStack(obj) {
  var value = $stack.pop();
  var name = $stack.pop();
  obj[name] = value;
}


// Arithmetic
function A() { }

A.prototype.a = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x + y);
};

A.prototype.s = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x - y);
};

A.prototype.m = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x * y);
};

A.prototype.d = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x / y);
};

A.prototype.mod = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x % y);
};

A.prototype.f = function() {
  var x = $stack.pop();

  $stack.push(Math.floor(x));
};

A.prototype.e = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x == y ? 1 : 0);
};

A.prototype.ne = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x != y ? 1 : 0);
};

A.prototype.lt = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x < y ? 1 : 0);
};

A.prototype.le = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x <= y ? 1 : 0);
};

A.prototype.gt = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x > y ? 1 : 0);
};

A.prototype.ge = function() {
  var y = $stack.pop();
  var x = $stack.pop();

  $stack.push(x >= y ? 1 : 0);
};


// Strings
function S() { }

S.prototype.l = function() {
  var string = $stack.pop();

  $stack.push(string.length);
};

S.prototype.i = function() {
  var n = $stack.pop();
  var string = $stack.pop();

  $stack.push(string[n]);
};

S.prototype.si = function() {
  var char = $stack.pop();
  var n = $stack.pop();
  var string = $stack.pop();

  string[n] = char;
  $stack.push(string);
};

S.prototype.a = function() {
  var s2 = $stack.pop();
  var s1 = $stack.pop();

  $stack.push(s1 + s2);
};

S.prototype.d = function() {
  var pos = $stack.pop();
  var string = $stack.pop();

  $stack.push(string.slice(0, pos), string.slice(pos + 1));
};

S.prototype.e = function() {
  var s2 = $stack.pop();
  var s1 = $stack.pop();

  $stack.push(s1 == s2 ? 1 : 0);
};

S.prototype.ns = function() {
  var number = $stack.pop();

  $stack.push("" + number);
};

S.prototype.sn = function() {
  var character = $stack.pop();

  $stack.push(parseInt(character));
};


// TODO: Variables
function V() { }


// Output
function O() { }

O.prototype.o = function() {
  var string = $stack.pop();

  $output(string);
};

O.prototype.on = function() {
  var number = $stack.pop();

  $output(number + "");
};


// TODO: Input
function I() { }
