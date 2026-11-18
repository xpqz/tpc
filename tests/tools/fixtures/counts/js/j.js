// line comment
/* single-line block */
/*
 * multi
 * line
 */
/** doc block */
const a = 1; // trailing line comment
const b = 2; /* trailing block */
/* lead */ const c = 3;
   

function f() {
  // inside
  return a;
}
/* start
 end */ f();
