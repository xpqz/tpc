program TypeAliasScalars;
{ a type name for a scalar type is the same type: alias chains, var parameter identity, function
  results, a type section after a var section, and a type declared inside a routine }
const Limit = 100;
type Num = Integer; Idx = Num;
var a: Idx; i: Integer;
type Small = Byte;
var s: Small; b: Byte;
procedure Bump(var x: Num);
begin
  x := x + 1
end;
procedure BumpByte(var x: Byte);
begin
  x := x + 1
end;
function Half(x: Num): Idx;
begin
  Half := x div 2
end;
procedure Local;
type Wide = LongInt; Flag = Boolean;
var w: Wide; f: Flag;
begin
  w := 100000; w := w * 3;
  f := w > Limit;
  WriteLn(w, ' ', f)
end;
begin
  i := 1; Bump(i); a := i; Bump(a);
  WriteLn(i, ' ', a, ' ', Half(a));
  s := 250; BumpByte(s); b := s; BumpByte(b); BumpByte(b); BumpByte(s);
  WriteLn(s, ' ', b);
  Local
end.
